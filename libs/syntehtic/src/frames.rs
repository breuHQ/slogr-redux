//! The frame defines all the frames required for communicating.

mod accept;
mod accept_session;
mod ipvn;
mod mode;
mod request_session;
mod server_greeting;
mod server_start;
mod setup_response;
mod start_ack;
mod start_session;
mod stop_session;

use crate::errors::SyntheticError;

pub use self::{
  accept::Accept, accept_session::AcceptSessionFrame, ipvn::IpVn, mode::Mode, request_session::RequestSessionFrame,
  server_greeting::ServerGreetingFrame, server_start::ServerStartFrame, setup_response::SetupResponseFrame,
  start_ack::StartAckFrame, start_session::StartSessionFrame, stop_session::StopSessionFrame,
};

/// Protocol agnostic frame implementation. Represents all the possible information arrangements
#[derive(Debug, Clone, Copy)]
pub enum SyntheticFrame {
  /// represents the server greeting frame
  ServerGreeting(ServerGreetingFrame), // 64 bytes
  /// represents the setup response frame
  SetUpResponse(SetupResponseFrame), // 164 bytes
  /// represent the server start frame
  ServerStart(ServerStartFrame), // 48 Bytes
  /// represents request session frame
  RequestSession(RequestSessionFrame), // 96 bytes
  /// represents accept session frame
  AcceptSession(AcceptSessionFrame), // 48 bytes
  /// represents start session frame
  StartSession(StartSessionFrame), // 32 bytes
  /// represents start ack frame
  StartAck(StartAckFrame), // 32 bytes
  /// represents stop session frame
  StopSession(StopSessionFrame), // 32 bytes
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
      AcceptSessionFrame::SIZE => Ok(Self::AcceptSession(AcceptSessionFrame::from_bytes(bytes))),
      StartSessionFrame::SIZE => Ok(Self::StartSession(StartSessionFrame::from_bytes(bytes))),
      StartAckFrame::SIZE => Ok(Self::StartAck(StartAckFrame::from_bytes(bytes))),
      StopSessionFrame::SIZE => Ok(Self::StopSession(StopSessionFrame::from_bytes(bytes))),
      _ => Err(SyntheticError::IllegalFrame),
    }
  }
}
