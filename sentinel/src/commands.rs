extern crate twamp;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, trace, warn};
use twamp::controller::Controller as TwampController;

/// Bind on the port and listen for incoming connections.
pub async fn serve() {
  let listener = TcpListener::bind("0.0.0.0:9000".to_string()).await.expect("Cannot connect to port.");
  info!("Waiting for new connections on port: 9000");


  loop {
    let (stream, addr) = listener.accept().await.expect("Cannot accept connections.");
    info!("Accepted connection from {}", addr);

    tokio::spawn(async move {
      connect(stream).await;
    });
  }
  // let mut listener = TcpListener::bind("0.0.0.0:6666");

  // loop {
  //   let (stream, _) = listener.accept().await.unwrap();
  //   // A new task is spawned each time there is a new connection request
  //   tokio::spawn(async move {
  //     connect(stream).await;
  //   });
  // }
}

async fn connect(stream: TcpStream) {
  let connection = TwampController::new(stream);
}
