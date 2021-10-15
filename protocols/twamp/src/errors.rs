use thiserror::Error;

#[derive(Debug, Error)]
pub enum RfcError {
  #[error("Port already in use: {port:?}")]
  PortUnavailable { 
    port: String
   },

   #[error("Requested Mode: {request_mode:?}. Available Modes: {available_modes:?}.")]
   UnsupportedMode {
    request_mode: String,
    available_modes: Vec<String>,
   },

  #[error(transparent)]
  IOError(#[from] std::io::Error),
}
