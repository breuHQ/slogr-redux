use thiserror::Error;

#[derive(Debug, Error)]
pub enum TwampError {
  #[error("Port already in use: {port:?}")]
  PortUnavailable {port: String},
}
