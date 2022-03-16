#![warn(clippy::nursery, clippy::pedantic)]
const SERVER_DB_URL: &'static str = "file:db.sqlite"; //Where the sqlite db is.
const MAX_CHAT_BUFFER: usize = 16; //How many messages can be in a queue at a time.
mod commands;
mod db;
pub mod types;
#[macro_use]
extern crate diesel;
use futures_util::{sink::SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
use tokio_tungstenite as wshandler;
use wshandler::tungstenite::Message;


#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap(); //If we can not bind to 8080 we should pamic
    let (tx, _) = broadcast::channel(MAX_CHAT_BUFFER);
    let mut conn_threads = vec![];
    let db_pool = db::establish_connection(SERVER_DB_URL).await;
    loop {
        if let Ok(client) = listener.accept().await {
            match wshandler::accept_async(client.0).await {
                Ok(wsstream) => conn_threads.push(tokio::spawn(client_init(
                    wsstream,
                    tx.clone(),
                    db_pool.clone(), //If db connections can not be spawned then the server is should exit
                ))),
                Err(e) => eprintln!("Connection error with client at {} [{e}]", client.1),
            }
        }
    }
}
async fn client_init(
    conn: wshandler::WebSocketStream<TcpStream>,
    txch: broadcast::Sender<crate::types::ChannelMes>,
    db_conn: db::PoolType,
) -> anyhow::Result<()> {
    let (mut tx, mut rx) = conn.split();
    let creds = rx.next().await.unwrap()?;
    let userdata: types::LoginUser = serde_json::from_str(&creds.to_string())?;
    let user = match db::auth_user(db_conn.get().await.unwrap(), userdata).await{
        Some(e) => e,
        None => return Ok(()),
    };
    tx.send(Message::Text(format!("Logged in as {}", user.username)))
        .await?;
    let p = tokio::spawn(client_send(tx, txch.subscribe(), user.username.clone()));
    match client_recv(rx, txch, &user, db_conn).await {
        Ok(()) => println!("{} disconnected", user.username),
        Err(e) => eprintln!("{} disconnected reason: [{e}]", user.username),
    }
    p.abort();
    Ok(())
}

async fn client_recv(
    mut rx: types::SplitStream,
    txch: broadcast::Sender<crate::types::ChannelMes>,
    userdata: &types::LoggedInUser,
    dbconn: db::PoolType,
) -> anyhow::Result<()> {
    let commands = commands::Commands::new();
    while let Some(Ok(new_mes)) = rx.next().await {
        let username = userdata.username.clone();
        let unencoded: types::FromClient = serde_json::from_str(&new_mes.to_string())?;
        let mes = match commands.exec_command(dbconn.get().await.unwrap(), &unencoded, userdata.admin, username.clone()) {
            Ok(n) => n,
            Err(e) => types::ChannelMes { user: Some(username), data: e.to_string()},
        };
        txch.send(mes)?;
    }
    Ok(())
}

async fn client_send(mut conn: types::SplitSink, mut rx: broadcast::Receiver<crate::types::ChannelMes>, username: String) {
    loop {
        let latest_mes = rx.recv().await.unwrap(); //Disconnect clients if can not get from the client thingy
        if latest_mes.user == Some(username.clone()){
            conn.send(Message::Text(latest_mes.data)).await.unwrap();
        } else if latest_mes.user == None {
             conn.send(Message::Text(latest_mes.data)).await.unwrap();
        }
    }
}
