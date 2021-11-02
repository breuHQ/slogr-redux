//! Client side of the synthetic I/O.

use std::net::SocketAddr;

use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use tracing::debug;

use crate::{errors::SyntheticError, io::connection::Connection};

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
    let mut framed = BytesCodec::new().framed(stream);
    while let Some(frame) = framed.next().await {
      match frame {
        Ok(frame) => {
          debug!("bytes [{:?}] {:?}", frame.len(), frame);
          match frame.len() {
            64 => {
              let bytes = frame.to_vec();
              debug!("decoded {:?}", bytes);
            }
            164 => {
              let bytes = frame.to_vec();
              debug!("decoded {:?}", bytes);
            }
            _ => debug!("unknown length"),
          }
          // return Ok(());
        }
        Err(e) => {
          return Err(SyntheticError::IOError(e));
        }
      }
    }
    debug!("Do I reach here");
    Err(SyntheticError::IllegalFrame)
  }
}
