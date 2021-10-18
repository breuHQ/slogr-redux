use std::net::SocketAddr;

use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};

use crate::io::connection::Connection;

pub struct Client {
  pub connection: Connection,
}

impl Client {
  pub async fn connect() -> Result<(), std::io::Error> {
    let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let stream = TcpStream::connect(&addr).await?;
    let connection = Connection::new(stream, addr);
    let mut framed = BytesCodec::new().framed(connection.stream);

    Ok(while let Some(message) = framed.next().await {
      match message {
        Ok(bytes) => println!("bytes: {:?}", bytes),
        Err(err) => println!("Socket closed with error: {:?}", err),
      }
    })
  }
}
