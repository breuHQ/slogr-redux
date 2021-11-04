//! When our agent is serving as a server, we need to be able to send messages to the client.
use futures::sink::SinkExt;
use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::debug;

use crate::{
  codec::SyntheticFrameCodec,
  errors::SyntheticError,
  frames::{Mode, ServerGreetingFrame, SetupResponseFrame, SyntheticFrame},
  io::connection::Connection,
};

/// Defines the server as per the RFC definition.
#[derive(Debug)]
pub struct Server {
  addr: SocketAddr,
  listener: TcpListener,
}

/// Represents a single connection to the server.
impl Server {
  /// Starts a new server.
  pub async fn run() -> Result<TcpListener, SyntheticError> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    // .expect(msg!("Failed to bind to port 9000"));
    debug!("Server started on port 9000");

    loop {
      let (stream, addr) = listener.accept().await?;
      let connection = Connection::new(stream, addr);
      tokio::spawn(Server::handle(connection));
    }
  }

  /// handles the connection
  async fn handle(connection: Connection) -> Result<(), SyntheticError> {
    let mut framed = Framed::new(connection.stream, SyntheticFrameCodec::new());
    let mode = Mode::Unauthenticated;
    framed
      .send(SyntheticFrame::ServerGreeting(ServerGreetingFrame::with_mode(mode)))
      .await?;
    // I should be able to different frames and get it decoded.
    framed
      .send(SyntheticFrame::SetUpResponse(SetupResponseFrame::with_mode(mode)))
      .await?;

    while let Some(message) = framed.next().await {
      match message {
        Ok(bytes) => println!("bytes: {:?}", bytes),
        Err(err) => Err(err)?,
      }
    }

    Ok(())
  }
}
