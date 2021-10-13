extern crate twamp;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use tracing::{debug, info};
use twamp::controller::Controller as TwampController;

/// Bind on the port and listen for incoming connections.
pub async fn serve() {
  let listener = TcpListener::bind("0.0.0.0:9000".to_string())
    .await
    .expect("Cannot connect to port.");
  info!("Waiting for new connections on port: 9000");

  loop {
    let (stream, addr) = listener.accept().await.expect("Port Already in use.");
    info!("Accepted connection from {}", addr);

    tokio::spawn(async move {
      // let mut controller = TwampController::new(stream);
      // // We're parsing each socket with the `BytesCodec` included in `tokio::codec`.
      let mut framed = BytesCodec::new().framed(stream);

      // We loop while there are messages coming from the Stream `framed`.
      // The stream will return None once the client disconnects.
      while let Some(message) = framed.next().await {
        match message {
          Ok(bytes) => info!("bytes: {:?}", bytes),
          Err(err) => info!("Socket closed with error: {:?}", err),
        }
      }
      println!("Socket received FIN packet and closed connection");
    });
  }
}
