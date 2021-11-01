//! Connection to a remote peer

use std::net::SocketAddr;

use tokio::{
  io::{AsyncWriteExt, BufWriter},
  net::TcpStream,
};
use tracing::{debug, info};

use crate::{
  errors::SyntheticError,
  frames::{Mode, ServerGreetingFrame, SetupResponseFrame},
};

/// Represents a connection to the underlying stream.
/// Depending on the role of the connection i.e. the server, control client, session sender or session reflector, we
/// instantiate our connection.
#[derive(Debug)]
pub struct Connection {
  /// underlying tcp stream wrapped inside a buffer for writing
  pub stream: BufWriter<TcpStream>,
  /// the remote address for the connection
  pub addr: SocketAddr,
  /// the mode the server is connected on
  pub mode: Mode,
}

impl Connection {
  /// creates a new connection object
  pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
    info!("Established connection for: {:?}", addr);
    let stream = BufWriter::new(stream);
    let mode = Mode::Unauthenticated; // TODO: Get this from global configuration.

    Self { stream, addr, mode }
  }

  /// Sends a server greeting frame from the server
  pub async fn send_server_greeting(&mut self) -> Result<(), std::io::Error> {
    let bytes = ServerGreetingFrame::with_mode(self.mode).to_bytes();
    let bytes = bytes.as_slice();
    debug!("Sending server greeting frame: {:?}", bytes);
    self.stream.write_all(bytes).await?;
    self.stream.flush().await?;
    info!("Finished sending server greeting frame");

    Ok(())
  }

  /// Reads the server greeting frame from the server
  pub async fn read_server_greeting(&self) -> Result<Self, SyntheticError> {
    todo!();
  }

  /// Sends the setup response frame to the server.
  pub async fn send_setup_response(&mut self) -> Result<(), std::io::Error> {
    let bytes = SetupResponseFrame::with_mode(self.mode).to_bytes();
    let bytes = bytes.as_slice();

    info!("Sending setup response frame");
    self.stream.write_all(&bytes).await?;
    self.stream.flush().await?;
    Ok(())
  }
}
