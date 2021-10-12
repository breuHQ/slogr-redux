use bytes::{Buf, BytesMut};
use std::io::{self, Cursor};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

/// To provide write level buffering, we use a BufWriter and wrap the underlying stream.
#[derive(Debug)]
pub struct Controller {
  stream: BufWriter<TcpStream>,
  buffer: BytesMut,
  // cursor: Cursor<BytesMut>,
}

impl Controller {
  pub fn new(_stream: TcpStream) -> Self {
    Self {
      stream: BufWriter::new(_stream),
      buffer: BytesMut::with_capacity(4 * 1024), // TODO: experimentally determine it.
    }
  }

  pub async fn handshake(&mut self) {}

  pub async fn read_frame(&mut self) {}

  pub async fn parse_frame(&mut self) {}

  pub async fn write_frame(&mut self) {}
}
