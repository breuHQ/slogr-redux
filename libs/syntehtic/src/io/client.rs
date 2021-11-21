//! Client side of the synthetic I/O.

use futures::{Future, SinkExt};
use std::{net::SocketAddr, pin::Pin};

use eyre::Result;
use tokio_stream::StreamExt;
use tokio_util::codec::Decoder;

use crate::{
  codec::SyntheticFrameTCPCodec, errors::SyntheticError, frames::SyntheticFrame, io::connection::Connection,
};

// ysf: reducing the cognitive load on the type
type FramedStream = Result<SyntheticFrame, SyntheticError>;
type TimedFramedStream = Result<Option<FramedStream>, tokio::time::error::Elapsed>;

/// Serves as a container for the client connection.
#[derive(Debug)]
pub struct Client {
  /// Represents a tcp connection
  pub connection: Connection,
}

impl Client {
  /// connect to a given address
  pub async fn connect() -> Result<(), SyntheticError> {
    let timeout_strategy = tokio::time::Duration::from_secs(120);
    let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let stream = tokio::net::TcpStream::connect(addr).await?;
    let mut stream = SyntheticFrameTCPCodec::new().framed(stream);
    let stream_with_timeout = tokio::time::timeout(timeout_strategy, stream.next());
    let pinned_stream: Pin<Box<dyn Future<Output = TimedFramedStream>>> = Box::pin(stream_with_timeout);

    if let Ok(Some(response)) = pinned_stream.await {
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
