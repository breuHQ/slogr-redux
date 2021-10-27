//! Defines errors across all crate
use thiserror::Error;

/// An interface to define errors accross the entire library
#[derive(Debug, Error)]
pub enum SyntheticError {
  /// An error when the port is unavilable
  #[error("Port already in use: {port:?}")]
  PortUnavailable {
    /// The port that is unavailable 
    port: String 
  },

  /// The requested greeting mode is not avilable for the given server
  #[error("Requested Mode: {request_mode:?}. Available Modes: {available_modes:?}.")]
  UnsupportedMode {
    /// the mode with which the connection request is made
    request_mode: String,
    /// available modes for the given server
    available_modes: Vec<String>,
  },

  /// Error while trying to convert bytes to frame
  #[error("Invalid or empty frame")]
  InvalidOrEmptyFrame,

  /// transparently communicating back errors
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}
