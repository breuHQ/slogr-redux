use std::net::SocketAddr;

use bytes::BytesMut;
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt, BufWriter},
  net::TcpStream,
};

#[derive(Debug)]
pub struct Connection {
  pub stream: BufWriter<TcpStream>,
  pub addr: SocketAddr,
  pub buffer: BytesMut,
  pub cursor: usize,
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

  pub async fn send_server_greeting(&self) -> Result<Self, std::io::Error> {
    // let stream = self.stream.clone();
    todo!();
  }

  pub fn read_server_greeting(&self) -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn send_setup_response(&self) -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn read_setup_response(&self) -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn send_server_start(&self) -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn read_server_start(&self) -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn send_request_session(&self) -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn read_request_session(&self) -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn send_accept_session(&self) -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn receive_accept_session(&self) -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn send_start_session() -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn receive_start_session() -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn send_start_ack() -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn receive_start_ack() -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn send_stop_sessions() -> Result<Self, std::io::Error> {
    todo!();
  }

  pub fn receive_stop_sessions() -> Result<Self, std::io::Error> {
    todo!();
  }
}
