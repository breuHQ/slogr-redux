//! Synthtehtic codec
use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::{
  errors::SyntheticError,
  frames::{RequestSessionFrame, ServerGreetingFrame, ServerStartFrame, SetupResponseFrame, SyntheticFrame},
};

/// Codec for [SyntehticFrame]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct SyntheticFrameCodec(());

impl SyntheticFrameCodec {
  /// Creates a new instance of the codec
  pub fn new() -> Self {
    SyntheticFrameCodec(())
  }
}

impl Decoder for SyntheticFrameCodec {
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

    let frame = SyntheticFrame::try_from_bytes(payload);
    match frame {
      Ok(frame) => Ok(Some(frame)),
      Err(err) => Err(err),
    }
  }
}

impl Encoder<SyntheticFrame> for SyntheticFrameCodec {
  type Error = SyntheticError;

  fn encode(&mut self, frame: SyntheticFrame, dst: &mut BytesMut) -> Result<(), Self::Error> {
    match frame {
      SyntheticFrame::ServerGreeting(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(ServerGreetingFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::SetUpResponse(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(SetupResponseFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::ServerStart(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(ServerStartFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::RequestSession(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(RequestSessionFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::AcceptSession(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(ServerStartFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::StartSession(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(ServerStartFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::StartAck(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(ServerStartFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::StopSession(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(ServerStartFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
    }
  }
}
