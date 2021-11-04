//! The frame defines all the frames required for communicating.

mod accept;
mod ipvn;
mod mode;
mod request_session;
mod server_greeting;
mod server_start;
mod setup_response;

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

impl SyntheticFrame {
  /// Tries to convert from a bytes array
  pub fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, SyntheticError> {
    let size = bytes.len();
    match size {
      ServerGreetingFrame::SIZE => Ok(Self::ServerGreeting(ServerGreetingFrame::from_bytes(bytes))),
      SetupResponseFrame::SIZE => Ok(Self::SetUpResponse(SetupResponseFrame::from_bytes(bytes))),
      ServerStartFrame::SIZE => Ok(Self::ServerStart(ServerStartFrame::from_bytes(bytes))),
      RequestSessionFrame::SIZE => Ok(Self::RequestSession(RequestSessionFrame::from_bytes(bytes))),
      _ => Err(SyntheticError::IllegalFrame),
    }
  }
}
