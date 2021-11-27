//! When our agent is serving as a server, we need to be able to send messages to the client.
use futures::sink::SinkExt;
use std::net::SocketAddr;

use eyre::Result;
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{info, instrument};

use crate::{
  codec::SyntheticFrameTCPCodec,
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
    let now = chrono::Utc::now().timestamp_nanos();
    info!("Server [Timestamp]: {}", now);
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    // .expect(msg!("Failed to bind to port 9000"));
    info!("Server [LISTENING]: {:?}", listener.local_addr().unwrap());

    loop {
      let (stream, addr) = listener.accept().await?;
      let connection = Connection::new(stream, addr);
      tokio::task::spawn(Server::handle(connection));
    }
  }

  /// handles the connection
  #[instrument]
  async fn handle(connection: Connection) -> Result<(), SyntheticError> {
    let mode = Mode::Authenticated;
    info!("Connection [NEW]: {:?}", connection.addr);

    let mut framed_stream = Framed::new(connection.stream, SyntheticFrameTCPCodec::new());
    framed_stream
      .send(SyntheticFrame::ServerGreeting(ServerGreetingFrame::new(mode)))
      .await?;

    // _quick_concurrency_check(mode, &mut framed).await?;

    while let Some(response) = framed_stream.next().await {
      match response {
        Ok(frame) => println!("frame: {:?}", frame),
        Err(error) => return Err(error),
      }
    }

    // TODO: Here we need to remove the connection from the list of connections.
    info!("Connection [CLOSED] {:?}", connection.addr);

    Ok(())
  }
}

async fn _quick_concurrency_check(
  mode: Mode,
  stream: &mut Framed<tokio::io::BufWriter<tokio::net::TcpStream>, SyntheticFrameTCPCodec>,
) -> Result<(), SyntheticError> {
  let mut frames: Vec<SyntheticFrame> = Vec::new();
  for _ in 1..10000 {
    frames.push(SyntheticFrame::ServerGreeting(ServerGreetingFrame::new(mode)));
    frames.push(SyntheticFrame::SetUpResponse(SetupResponseFrame::new(mode)));
  }
  for frame in frames {
    stream.send(frame).await?;
  }
  Ok(())
}
