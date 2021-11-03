//! The frame defines all the frames required for communicating.

mod accept;
mod ipvn;
mod mode;
mod request_session;
mod server_greeting;
mod server_start;
mod setup_response;

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use tracing::debug;

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
    // Not enough data to decode
    if src.len() < 2 {
      return Ok(None);
    }

    // Get delimiter
    let mut delimiter: [u8; 2] = [0; 2];
    delimiter.copy_from_slice(&src[..2]);
    debug!("---");
    debug!("First two bytes: {:?}", delimiter);

    let size: usize = u16::from_be_bytes(delimiter) as usize;
    let data = src[2..(size + 2)].to_vec();
    debug!("Data: {:?}", data);
    src.advance(size + 2);
    debug!("Remaining {:?}", src.len());

    if size == 0 {
      return Ok(None);
    }

    match size {
      ServerGreetingFrame::SIZE => Ok(Some(SyntheticFrame::ServerGreeting(ServerGreetingFrame::from_bytes(
        data,
      )))),
      SetupResponseFrame::SIZE => Ok(Some(SyntheticFrame::SetUpResponse(SetupResponseFrame::from_bytes(
        data,
      )))),
      ServerStartFrame::SIZE => Ok(Some(SyntheticFrame::ServerStart(ServerStartFrame::from_bytes(data)))),
      RequestSessionFrame::SIZE => Ok(Some(SyntheticFrame::RequestSession(RequestSessionFrame::from_bytes(
        data,
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
        let delimter: Vec<u8> = frame.get_delimiter();
        debug!("Delimiter: {:?}", delimter);
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(ServerGreetingFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::SetUpResponse(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        debug!("Delimiter: {:?}", delimter);
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(SetupResponseFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::ServerStart(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        debug!("Delimiter: {:?}", delimter);
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(ServerStartFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
      SyntheticFrame::RequestSession(frame) => {
        let delimter: Vec<u8> = frame.get_delimiter();
        debug!("Delimiter: {:?}", delimter);
        dst.reserve(2);
        dst.put(delimter.as_slice());
        dst.reserve(RequestSessionFrame::SIZE);
        dst.put(frame.to_bytes().as_slice());
        Ok(())
      }
    }
  }
}
