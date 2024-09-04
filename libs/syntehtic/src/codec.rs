//! Synthtehtic codec
use bytes::{Buf, BufMut, BytesMut};
use eyre::Result;
use tokio_util::codec::{Decoder, Encoder};

use crate::{
  errors::SyntheticError,
  frames::{
    self, AcceptSessionFrame, RequestSessionFrame, ServerGreetingFrame, ServerStartFrame, SetupResponseFrame,
    StartSessionFrame, SyntheticFrame,
  },
  macros::write_frame,
};

/// Codec for [`SyntheticFrame`]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct SyntheticFrameTCPCodec(());

impl SyntheticFrameTCPCodec {
  /// Creates a new instance of the codec
  pub fn new() -> Self {
    SyntheticFrameTCPCodec(())
  }
}

impl Decoder for SyntheticFrameTCPCodec {
  type Item = SyntheticFrame;
  type Error = SyntheticError;

  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    // Not enough header bytes
    if src.len() < 2 {
      return Ok(None);
    }

    // Get the length of the frame
    let mut delimiter: [u8; 2] = [0; 2];
    delimiter.copy_from_slice(&src[..2]);
    let size: usize = u16::from_be_bytes(delimiter) as usize;

    // The entire frame has not arrived yet
    if src.len() < size {
      return Ok(None);
    }

    // Get the payload
    let payload: Vec<u8> = src[2..(size + 2)].to_vec();
    src.advance(size + 2);

    if size == 0 {
      return Ok(None);
    }

    match SyntheticFrame::try_from(payload) {
      Ok(frame) => Ok(Some(frame)),
      Err(err) => Err(err),
    }
  }
}

impl Encoder<SyntheticFrame> for SyntheticFrameTCPCodec {
  type Error = SyntheticError;

  fn encode(&mut self, frame: SyntheticFrame, destination: &mut BytesMut) -> Result<(), Self::Error> {
    match frame {
      SyntheticFrame::ServerGreeting(frame) => write_frame!(frame, destination, ServerGreetingFrame::SIZE),
      SyntheticFrame::SetUpResponse(frame) => write_frame!(frame, destination, SetupResponseFrame::SIZE),
      SyntheticFrame::ServerStart(frame) => write_frame!(frame, destination, ServerStartFrame::SIZE),
      SyntheticFrame::RequestSession(frame) => write_frame!(frame, destination, RequestSessionFrame::SIZE),
      SyntheticFrame::AcceptSession(frame) => write_frame!(frame, destination, AcceptSessionFrame::SIZE),
      SyntheticFrame::StartSession(frame) => write_frame!(frame, destination, StartSessionFrame::SIZE),
      SyntheticFrame::StartAck(frame) => write_frame!(frame, destination, frames::StartAckFrame::SIZE),
      SyntheticFrame::StopSession(frame) => write_frame!(frame, destination, frames::StopSessionFrame::SIZE),
    }
  }
}
