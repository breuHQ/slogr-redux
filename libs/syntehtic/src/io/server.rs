use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use tracing::debug;

use crate::io::connection::Connection;

/// Defines the server as per the RFC definition.
#[derive(Debug)]
pub struct Server {
  addr: SocketAddr,
  listener: TcpListener,
}

/// Represents a single connection to the server.
impl Server {
  // Starts a new server.
  pub async fn run() -> Result<TcpListener, std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    // .expect(msg!("Failed to bind to port 9000"));
    debug!("Server started on port 9000");

    loop {
      let (stream, addr) = listener.accept().await.unwrap();
      let connection = Connection::new(stream, addr);
      Server::handle(connection).await?;
    }
  }

  /// handles the connection
  async fn handle(mut connection: Connection) -> Result<(), std::io::Error> {
    let result = tokio::spawn(async move {
      connection.send_server_greeting().await?;
      connection.send_setup_response().await?;

      let mut framed = BytesCodec::new().framed(connection.stream);
      while let Some(message) = framed.next().await {
        match message {
          Ok(bytes) => println!("bytes: {:?}", bytes),
          Err(err) => println!("Socket closed with error: {:?}", err),
        }
      }
      Ok(())
      // loop {
      //   let _frame = connection.read_frame().await?;
      //   match _frame {
      //     Frame::SetUpResponse(_) => todo!(), // TODO: only match all the possible frames we can get then process accordingly.
      //     Frame::ServerStart(_) => todo!(),
      //     _ => todo!(), // TODO: replace with raising error
      //   }
      // }
    });
    result.await?
  }
}
