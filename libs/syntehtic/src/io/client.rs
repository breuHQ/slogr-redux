//! Client side of the synthetic I/O.

use futures::{Future, SinkExt};
use std::{net::SocketAddr, pin::Pin};
use tracing::info;

use eyre::Result;
use tokio_stream::StreamExt;
use tokio_util::codec::Decoder;

use crate::{
  codec::SyntheticFrameTCPCodec,
  common::{Reply, SendSyncStatic},
  errors::SyntheticError,
  frames::SyntheticFrame,
  io::connection::Connection,
};

// ysf: reducing the cognitive load on the type
type SyntheticStreamResult = Result<SyntheticFrame, SyntheticError>;
type SyntheticStreamResultWithTimeout = Result<Option<SyntheticStreamResult>, tokio::time::error::Elapsed>;
type PinnedSyntheticStreamResultWithTimeout = Pin<Box<dyn Future<Output = SyntheticStreamResultWithTimeout>>>;

/// Serves as a container for the client connection.
#[derive(Debug)]
pub struct Client {
  /// Represents a tcp connection
  pub connection: Connection,
}

impl SendSyncStatic for Client {}

impl Client {
  /// connect to a given address
  pub async fn connect() -> Result<(), SyntheticError> {
    let timeout_strategy = tokio::time::Duration::from_secs(120);
    let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let stream = tokio::net::TcpStream::connect(addr).await?;
    info!("Connected: {}", addr);
    let mut stream = SyntheticFrameTCPCodec::new().framed(stream);
    let stream_with_timeout = tokio::time::timeout(timeout_strategy, stream.next());
    let pinned_stream: Pin<Box<dyn Future<Output = SyntheticStreamResultWithTimeout>>> = Box::pin(stream_with_timeout);

    if let Ok(Some(response)) = pinned_stream.await {
      Client::handle(response, stream).await?;
    }

    Ok(())
  }

  async fn handle(
    response: Result<SyntheticFrame, SyntheticError>,
    mut stream: tokio_util::codec::Framed<tokio::net::TcpStream, SyntheticFrameTCPCodec>,
  ) -> Result<(), SyntheticError> {
    match response {
      Ok(resp) => {
        // ysf: this should be `response.reply(stream).await?`.
        match resp {
          SyntheticFrame::ServerGreeting(f) => stream.send(SyntheticFrame::SetUpResponse(f.reply())).await?,
          SyntheticFrame::SetUpResponse(_) => todo!(),
          SyntheticFrame::ServerStart(_) => todo!(),
          SyntheticFrame::RequestSession(_) => todo!(),
          SyntheticFrame::AcceptSession(_) => todo!(),
          SyntheticFrame::StartSession(_) => todo!(),
          SyntheticFrame::StartAck(_) => todo!(),
          SyntheticFrame::StopSession(_) => todo!(),
        }
      }
      Err(err) => return Err(err),
    };
    Ok(())
  }
}
