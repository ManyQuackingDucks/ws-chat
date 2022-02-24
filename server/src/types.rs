use serde::Deserialize;
use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

pub type SplitSink =
    futures_util::stream::SplitSink<WebSocketStream<TcpStream>, Message>;

pub type SplitStream = futures_util::stream::SplitStream<WebSocketStream<TcpStream>>;

#[derive(Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

impl LoginUser{
    fn get_user(self) -> String{
        self.username
    }
    #[allow(dead_code)]
    fn get_pass(self) -> String{
        self.password
    }
}

pub struct User {
    pub username: String,
    pub perm_level: u8,
}

impl User{
    pub fn new(user: LoginUser) -> anyhow::Result<Self>{
        //Authenticate
        let perm_level = 0;
        Ok(Self {username: user.get_user(), perm_level})
    }
}