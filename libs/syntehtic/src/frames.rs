//! The frame defines all the frames required for communicating.

mod accept;
mod ipvn;
mod mode;
mod request_session;
mod server_greeting;
mod server_start;
mod setup_response;

use bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::errors::SyntheticError;

pub use self::{
  accept::Accept, ipvn::IpVn, mode::Mode, request_session::RequestSessionFrame, server_greeting::ServerGreetingFrame,
  server_start::ServerStartFrame, setup_response::SetupResponseFrame,
};

/// Protocol agnostic frame implementation. Represents all the possible information arrangements
#[derive(Debug, Clone, Copy)]
pub enum SyntheticFrame {
  /// represents the server greeting frame
  ServerGreeting(ServerGreetingFrame),
  /// represents the setup response frame
  SetUpResponse(SetupResponseFrame),
  /// represent the server start frame
  ServerStart(ServerStartFrame),
  /// represents request session frame
  RequestSession(RequestSessionFrame),
}

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
    let size = src.len();
    let bytes = src.split_to(size);
    let bytes = bytes.to_vec();

    if size < 1 {
      return Ok(None);
    }

    match size {
      ServerGreetingFrame::SIZE => Ok(Some(SyntheticFrame::ServerGreeting(ServerGreetingFrame::from_bytes(
        bytes,
      )))),
      SetupResponseFrame::SIZE => Ok(Some(SyntheticFrame::SetUpResponse(SetupResponseFrame::from_bytes(
        bytes,
      )))),
      ServerStartFrame::SIZE => Ok(Some(SyntheticFrame::ServerStart(ServerStartFrame::from_bytes(bytes)))),
      RequestSessionFrame::SIZE => Ok(Some(SyntheticFrame::RequestSession(RequestSessionFrame::from_bytes(
        bytes,
      )))),
      _ => Err(SyntheticError::IllegalFrame),
    }
  }
}

impl Encoder<SyntheticFrame> for SyntheticFrameCodec {
  type Error = SyntheticError;

  fn encode(&mut self, frame: SyntheticFrame, dst: &mut BytesMut) -> Result<(), Self::Error> {
    match frame {
      SyntheticFrame::ServerGreeting(frame) => {
        dst.reserve(ServerGreetingFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::SetUpResponse(frame) => {
        dst.reserve(SetupResponseFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::ServerStart(frame) => {
        dst.reserve(ServerStartFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::RequestSession(frame) => {
        dst.reserve(RequestSessionFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
    }
  }
}
