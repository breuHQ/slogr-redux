use std::{io::Error, net::SocketAddr};

use bytes::BytesMut;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use tracing::{debug, error};

use crate::{errors::SyntheticError, io::connection::Connection};

pub struct Client {
  pub connection: Connection,
}

impl Client {
  /// picks the connection from either outpost, or default.
  // pub async fn connect() -> Result<(), std::io::Error> {
  //   let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
  //   let stream = TcpStream::connect(addr).await?;
  //   let mut framed = BytesCodec::new().framed(stream);
  //   while let Some(message) = framed.next().await {
  //     match message {
  //       Ok(bytes) => debug!("bytes: {:?}", bytes),
  //       // Err(err) => error!("Socket closed with error: {:?}", err),
  //       Err(err) => error!("Parse Error: {:?}", err),
  //     }
  //   }

  //   Ok(())
  // }

  pub async fn connect() -> Result<(), SyntheticError> {
    let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let stream = TcpStream::connect(addr).await?;
    while let Some(frame) = BytesCodec::new().framed(stream).next().await {
      match frame {
        Ok(_) => { return Ok(()); },
        Err(e) => { return Err(SyntheticError::IOError(e)); }
      }
    }
    Err(SyntheticError::InvalidOrEmptyFrame)
  }

  pub async fn process(&self) {}
}
