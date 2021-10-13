use bytes::{Buf, BytesMut};
use std::fmt::Debug;
use std::io::{self, Cursor};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Decoder};

use crate::frames::Frame;

/// To provide write level buffering, we use a BufWriter and wrap the underlying stream.
#[derive(Debug)]
pub struct Controller {
  tcp_stream: BufWriter<TcpStream>,
  tcp_buffer: BytesMut,
  // cursor: Cursor<BytesMut>,
}

impl Controller {
  pub fn new(_stream: TcpStream) -> Self {
    Self {
      tcp_stream: BufWriter::new(_stream),
      tcp_buffer: BytesMut::with_capacity(4 * 1024), // TODO: experimentally determine it.
    }
  }

  pub async fn handshake(&mut self) {}

  // pub async fn read_frame(&mut self) -> Result<Option<Frame>, io::Error> {}

  pub async fn parse_frame(&mut self) {}

  pub async fn write_frame(&mut self) {}
}
