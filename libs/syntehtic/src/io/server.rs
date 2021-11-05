//! When our agent is serving as a server, we need to be able to send messages to the client.
use futures::sink::SinkExt;
use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{info, instrument};

use crate::{
  codec::SyntheticFrameCodecTCP,
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
  #[instrument]
  pub async fn run() -> Result<TcpListener, SyntheticError> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    // .expect(msg!("Failed to bind to port 9000"));
    info!("Server [LISTENING]: {:?}", listener.local_addr());

    loop {
      let (stream, addr) = listener.accept().await?;
      let connection = Connection::new(stream, addr);
      tokio::task::spawn(Server::handle(connection));
    }
  }

  /// handles the connection
  #[instrument]
  async fn handle(connection: Connection) -> Result<(), SyntheticError> {
    let mode = Mode::Unauthenticated;
    info!("Connection [NEW]: {:?}", connection.addr);

    let mut framed = Framed::new(connection.stream, SyntheticFrameCodecTCP::new());
    framed
      .send(SyntheticFrame::ServerGreeting(ServerGreetingFrame::with_mode(mode)))
      .await?;
    // I should be able to different frames and get it decoded.
    framed
      .send(SyntheticFrame::SetUpResponse(SetupResponseFrame::with_mode(mode)))
      .await?;

    // quick_concurrency_check(mode, &mut framed).await?;
    while let Some(message) = framed.next().await {
      match message {
        Ok(bytes) => println!("bytes: {:?}", bytes),
        Err(err) => Err(err)?,
      }
    }

    // TODO: Here we need to remove the connection from the list of connections.
    info!("Connection [CLOSED] {:?}", connection.addr);

    Ok(())
  }
}

async fn _quick_concurrency_check(
  mode: Mode,
  framed: &mut Framed<tokio::io::BufWriter<tokio::net::TcpStream>, SyntheticFrameCodecTCP>,
) -> Result<(), SyntheticError> {
  let mut frames: Vec<SyntheticFrame> = Vec::new();
  for _ in 1..10000 {
    frames.push(SyntheticFrame::ServerGreeting(ServerGreetingFrame::with_mode(mode)));
    frames.push(SyntheticFrame::SetUpResponse(SetupResponseFrame::with_mode(mode)));
  }
  Ok(for frame in frames {
    framed.send(frame).await?;
  })
}
