use bytes::BytesMut;
use std::net::SocketAddr;
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt, BufWriter},
  net::{TcpListener, TcpStream},
};
use tracing::debug;

use crate::frames::{Frame, ServerGreetingFrame};

/// Defines the server as per the RFC definition.
#[derive(Debug)]
pub struct Server {
  listener: TcpListener,
}

/// Represents a single connection to the server.
#[derive(Debug)]
pub struct Connection {
  stream: BufWriter<TcpStream>,
  addr: SocketAddr,
  buffer: BytesMut,
  cursor: usize,
}

impl Connection {
  pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
    let stream = BufWriter::new(stream);
    let buffer = BytesMut::with_capacity(4 * 1024 * 1024); // TODO: Determine this value. Currently 4MB.
    let cursor: usize = 0;

    Self {
      stream,
      addr,
      buffer,
      cursor,
    }
  }

  pub fn read_frame(&mut self) {}
  // pub fn write_frame() -> Result<(), TwampError> {}
}

impl Server {
  pub async fn run() {
    let listener = TcpListener::bind("0.0.0.0:9000")
      .await
      // .map_err(|_| TwampError::PortUnavailable { port: "9000".to_string() })
      .unwrap();

    debug!("Server started on port 9000");

    loop {
      let (stream, addr) = listener.accept().await.unwrap();
      let connection = Connection::new(stream, addr);
      debug!("Connection established with {}", addr);

      tokio::spawn(async move {
        let mut connection = connection;
        let frame = Frame::ServerGreeting(ServerGreetingFrame::mode_unauthenticated());
        match frame {
          Frame::ServerGreeting(frame) => {
            debug!("ServerGreetingFrame: {:?}", frame);
            debug!("ServerGreetingFrame [unused]: {:?}", frame.unused);
            connection.stream.write_all(b"!").await.unwrap();
            connection.stream.write_all(&frame.unused).await.unwrap();
            debug!("ServerGreetingFrame [mode]: {:?}", (frame.mode as u32).to_be_bytes());
            connection
              .stream
              .write_all(&(frame.mode as u32).to_be_bytes())
              .await
              .unwrap();
            debug!("ServerGreetingFrame [challenge]: {:?}", frame.challenge);
            connection.stream.write_all(&frame.challenge).await.unwrap();
            debug!("ServerGreetingFrame [salt]: {:?}", frame.salt);
            connection.stream.write_all(&frame.salt).await.unwrap();
            debug!("ServerGreetingFrame [count]: {:?}", (frame.count as u32).to_be_bytes());
            connection
              .stream
              .write_all(&(frame.count as u32).to_be_bytes())
              .await
              .unwrap();
            debug!("ServerGreetingFrame [mbz]: {:?}", frame.mbz);
            connection.stream.write_all(&frame.mbz).await.unwrap();
            connection.stream.write_all(b"\r\n").await.unwrap();
            connection.stream.flush().await.unwrap();
            debug!("ServerGreetingFrame: Finished writing");
          }
          _ => {}
        }
        loop {
          let n = connection.stream.read_buf(&mut connection.buffer).await.unwrap();
          debug!("Read {} bytes from {}", n, connection.addr);
        }
        // connection.read_frame();
      });

      // let (rx, tx) = stream.split();
      // debug!("New Connection: {}", addr);

      // let frame = Frame::ServerGreeting(ServerGreetingFrame::mode_unauthenticated());
      // let encoded = bincode::serialize(&frame).unwrap();
      // debug!("Encoded Frame: {:?}", encoded);
      // debug!("Encoded Frame Length: {}", encoded.len());

      // match frame {
      //   Frame::ServerGreeting(frame) => {
      //     debug!("Server Greeting Frame");
      //     // let mut buffer = BytesMut::with_capacity(4 * 1024);
      //     // frame.encode(&mut buffer);
      //     // stream.write_all(&buffer).await.unwrap();
      //   }
      //   Frame::SetUpResponse(_) => todo!(),
      //   Frame::ServerStart(_) => todo!(),
      // }

      // // Each connection spawns a new thread
      // tokio::spawn(async move {
      //   let mut framed = BytesCodec::new().framed(stream);

      //   // We loop while there are messages coming from the Stream `framed`.
      //   // The stream will return None once the client disconnects.
      //   while let Some(message) = framed.next().await {
      //     match message {
      //       Ok(bytes) => debug!("bytes: {:?}", bytes),
      //       Err(err) => debug!("Socket closed with error: {:?}", err),
      //     }
      //   }

      //   println!("Socket received FIN packet and closed connection");
      // });
    }

    // move the socket into a seperate parent thread.
    // this thread will be responsible to keep the server up and spawn a new process for each incoming connection
    // tokio::spawn(async move {
    //   debug!("Server thread started");
    // });

    // Ok(server)
  }
}
