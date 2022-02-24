use futures_util::{sink::SinkExt, StreamExt};
use tokio::{sync::broadcast, net::{TcpStream, TcpListener}};
use tokio_tungstenite as wshandler;
use wshandler::tungstenite::Message;

mod types;
const MAX_CHAT_BUFFER: usize = 16;


#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let (tx, _) = broadcast::channel(MAX_CHAT_BUFFER);
    let mut conn_threads = vec![];
    loop {
        let client = listener.accept().await.unwrap();
        match wshandler::accept_async(client.0).await {
            Ok(wsstream) => conn_threads.push(tokio::spawn(client_init(wsstream, tx.clone()))),
            Err(e) => eprintln!("Connection error with client at {} [{e}]", client.1),
        }
    }
}
async fn client_init(conn: wshandler::WebSocketStream<TcpStream>, txch: broadcast::Sender<String>) -> anyhow::Result<()>{
    let (mut tx, mut rx) = conn.split();
    let config = rx.next().await.unwrap()?;
    let userdata: types::LoginUser = serde_json::from_str(&config.to_string())?;
    let user = types::User::new(userdata).unwrap();
    tx.send(Message::Text(format!("Logged in as: {}", user.username))).await?;
    let p = tokio::spawn(client_send(tx, txch.subscribe()));
    match client_recv(rx, txch, &user).await {
        Ok(()) => println!("{} disconnected", user.username),
        Err(e) => eprintln!("{} disconnected reason: [{e}]", user.username),
    }
    p.abort();
    Ok(())
}

async fn client_recv(mut rx: types::SplitStream, txch: broadcast::Sender<String>, userdata: &types::User) -> anyhow::Result<()>{
    while let Some(Ok(new_mes)) = rx.next().await {
        txch.send(format!("{}: {}", userdata.username, new_mes))?;
    }
    Ok(())
}


async fn client_send(mut conn: types::SplitSink, mut rx: broadcast::Receiver<String>) {
    loop {
        let latest_mes = rx.recv().await.unwrap();
        conn.send(Message::Text(latest_mes)).await.unwrap();
    }

}
