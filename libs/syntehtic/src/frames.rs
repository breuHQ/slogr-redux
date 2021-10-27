//! The frame defines all the frames required for communicating.

mod accept;
mod ipvn;
mod mode;
mod request_session;
mod server_greeting;
mod server_start;
mod setup_response;

pub use self::{
  accept::Accept, ipvn::IpVn, mode::Mode, request_session::RequestSessionFrame,
  server_greeting::ServerGreetingFrame, server_start::ServerStartFrame, setup_response::SetupResponseFrame,
};

/// Protocol agnostic frame implementation. Represents all the possible information arrangements
#[derive(Debug, Clone, Copy)]
pub enum Frame {
  /// represents the server greeting frame
  ServerGreeting(ServerGreetingFrame),
  /// represents the setup response frame
  SetUpResponse(SetupResponseFrame),
  /// represent the server start frame
  ServerStart(ServerStartFrame),
  /// represents request session frame
  RequestSession(RequestSessionFrame),
}
