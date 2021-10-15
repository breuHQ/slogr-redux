use tokio::net::TcpListener;
use tracing::debug;

use crate::io::connection::Connection;

/// Defines the server as per the RFC definition.
#[derive(Debug)]
pub struct Server {
  listener: TcpListener,
}

/// Represents a single connection to the server.
impl Server {
  pub async fn start() -> Result<TcpListener, std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    // .expect(msg!("Failed to bind to port 9000"));
    debug!("Server started on port 9000");

    loop {
      let (stream, addr) = listener.accept().await.unwrap();
      let connection = Connection::new(stream, addr);
      Server::handle_connection(connection).await?;
    }
  }

  async fn handle_connection(connection: Connection) -> Result<(), std::io::Error> {
    let result = tokio::spawn(async move {
      connection.send_server_greeting().await?;
      // connection.read_setup_response().await?;
      Ok(())
    });
    result.await?
  }
}
