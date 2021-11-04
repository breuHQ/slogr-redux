//! Connection to a remote peer

use std::net::SocketAddr;

use tokio::{io::BufWriter, net::TcpStream};
use tracing::instrument;

use crate::frames::Mode;

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
  #[instrument]
  pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
    let stream = BufWriter::new(stream);
    let mode = Mode::Unauthenticated; // TODO: Get this from global configuration.

    Self { stream, addr, mode }
  }
}
