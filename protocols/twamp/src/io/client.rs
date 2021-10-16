use std::net::SocketAddr;

use tokio::net::TcpStream;

use crate::io::connection::Connection;

pub struct Client {
  pub stream: TcpStream,
}

impl Client {
  pub async fn connect() {
    let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let stream = TcpStream::connect(&addr).await.unwrap();
    let connection = Connection::new(stream, addr);
    connection.read_server_greeting().await.unwrap();
  }
}
