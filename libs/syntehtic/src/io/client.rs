//! Client side of the synthetic I/O.

use std::net::SocketAddr;

use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use tracing::debug;

use crate::{errors::SyntheticError, io::connection::Connection, frames::SynteticFrameCodec};

/// Serves as a container for the client connection.
#[derive(Debug)]
pub struct Client {
  /// Represents a tcp connection
  pub connection: Connection,
}

impl Client {
  /// connect to a given address
  pub async fn connect() -> Result<(), SyntheticError> {
    let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let stream = TcpStream::connect(addr).await?;
    let mut framed = SynteticFrameCodec::new().framed(stream);
    while let Some(frame) = framed.next().await {
      debug!("{:?}", frame);
    }
    Err(SyntheticError::IllegalFrame)
  }
}
