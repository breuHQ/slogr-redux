//! Client side of the synthetic I/O.

use std::net::SocketAddr;

use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Decoder;
use tracing::debug;

use crate::{codec::SyntheticFrameCodec, errors::SyntheticError, io::connection::Connection};

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
    let mut framed = SyntheticFrameCodec::new().framed(stream);

    let mut count = 1;

    while let Some(frame) = framed.next().await {
      match frame {
        Ok(f) => {
          debug!("Frame [{:?}]: {:?}", count, f);
          count += 1;
        }
        Err(err) => Err(err)?,
      }
    }
    Ok(())
  }
}
