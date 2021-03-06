use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub type SplitSink = futures_util::stream::SplitSink<WebSocketStream<TcpStream>, Message>;

pub type SplitStream = futures_util::stream::SplitStream<WebSocketStream<TcpStream>>;

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

pub struct LoggedInUser {
    pub username: String,
    pub admin: bool,
}

#[derive(Deserialize)]
pub struct FromClient {
    pub command: String,
    pub args: Vec<String>,
}
#[derive(Clone, Debug)]
pub struct ChannelMes {
    pub user: Option<String>,
    pub data: String,
}
