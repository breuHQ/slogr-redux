//! Client side of the synthetic I/O.

use std::net::SocketAddr;

use eyre::Result;
use futures::SinkExt;
use tokio_stream::StreamExt;
use tokio_util::codec::Decoder;

use crate::{
  codec::SyntheticFrameTCPCodec, errors::SyntheticError, frames::SyntheticFrame, io::connection::Connection,
};

/// Serves as a container for the client connection.
#[derive(Debug)]
pub struct Client {
  /// Represents a tcp connection
  pub connection: Connection,
}

impl Client {
  /// connect to a given address
  pub async fn connect() -> Result<(), SyntheticError> {
    let duration = tokio::time::Duration::from_secs(120);
    let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let stream = tokio::net::TcpStream::connect(addr).await?;
    let mut stream = SyntheticFrameTCPCodec::new().framed(stream);
    let timeout = tokio::time::timeout(duration, stream.next());

    if let Ok(Some(response)) = timeout.await {
      Client::process_server_greeting_frame(response, stream).await?;
    }

    Ok(())
  }

  async fn process_server_greeting_frame(
    response: Result<SyntheticFrame, SyntheticError>,
    mut stream: tokio_util::codec::Framed<tokio::net::TcpStream, SyntheticFrameTCPCodec>,
  ) -> Result<(), SyntheticError> {
    match response {
      Ok(frame) => {
        if let SyntheticFrame::ServerGreeting(greeting) = frame {
          stream
            .send(SyntheticFrame::SetUpResponse(greeting.generate_response()))
            .await?
        } else {
          return Err(SyntheticError::UnexpectedFrame);
        }
      }
      Err(err) => return Err(err),
    };
    Ok(())
  }
}
