use std::net::SocketAddr;

use bincode::Options;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use tracing::debug;

use crate::{errors::SyntheticError, io::connection::Connection};

#[derive(Debug)]
pub struct Client {
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
              let _options = bincode::DefaultOptions::new().with_big_endian();
              debug!("appears to be server greeting frame");
              let bytes = frame.to_vec();
              debug!("bytes: {:?}", bytes);
              // let decoded: ServerGreetingFrame = options.deserialize(&bytes[..]).unwrap();
              // debug!("decoded {:?}", decoded);
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
    Err(SyntheticError::InvalidOrEmptyFrame)
  }
}
