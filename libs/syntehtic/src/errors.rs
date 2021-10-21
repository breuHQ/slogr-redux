//! Defines errors across all crate
use thiserror::Error;

/// An interface to define errors accross the entire library
#[derive(Debug, Error)]
pub enum SyntheticError {
  #[error("Port already in use: {port:?}")]
  PortUnavailable { port: String },

  #[error("Requested Mode: {request_mode:?}. Available Modes: {available_modes:?}.")]
  UnsupportedMode {
    request_mode: String,
    available_modes: Vec<String>,
  },

  #[error("Invalid or empty frame")]
  InvalidOrEmptyFrame,

  #[error(transparent)]
  IOError(#[from] std::io::Error),
}
