use std::{net::SocketAddr, sync::Arc};

use bytes::BytesMut;
use tokio::{
  io::{AsyncWriteExt, BufWriter},
  net::TcpStream,
};
use tracing::{debug, info};

use crate::{errors::RfcError, frames::{ServerGreetingFrame, ServerGreetingMode, SetupResponseFrame}};

#[derive(Debug)]
pub struct Connection {
  pub stream: BufWriter<TcpStream>,
  pub addr: SocketAddr,
  pub buffer: BytesMut,
  pub cursor: usize,
  pub mode: ServerGreetingMode,
}

impl Connection {
  pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
    info!("Established connection for: {:?}", addr);
    let stream = BufWriter::new(stream);
    let buffer = BytesMut::with_capacity(4 * 1024 * 1024); // TODO: Determine this value. Currently 4MB.
    let cursor: usize = 0;
    let mode = ServerGreetingMode::Unauthenticated; // TODO: Get this from global configuration.

    Self {
      stream,
      addr,
      buffer,
      cursor,
      mode,
    }
  }

  /// Sends a server greeting frame from the server 
  pub async fn send_server_greeting(&mut self) -> Result<(), std::io::Error> {
    let frame = ServerGreetingFrame::with_mode(self.mode);

    info!("Server greeting mode: {:?}", self.mode);
    info!("Sending server greeting frame");
    
    debug!("ServerGreetingFrame [unused]: {:?}", frame.unused);
    self.stream.write_all(&frame.unused).await?;
    
    debug!("ServerGreetingFrame [mode]: {:?}", (frame.mode as u32).to_be_bytes());
    self.stream.write_all(&(self.mode as u32).to_be_bytes()).await?;

    debug!("ServerGreetingFrame [challenge]: {:?}", frame.challenge);
    self.stream.write_all(&frame.challenge).await?;
    
    debug!("ServerGreetingFrame [salt]: {:?}", frame.salt);
    self.stream.write_all(&frame.salt).await?;
    
    debug!("ServerGreetingFrame [count]: {:?}", (frame.count as u32).to_be_bytes());
    self.stream.write_all(&(frame.count as u32).to_be_bytes()).await?;
    
    debug!("ServerGreetingFrame [mbz]: {:?}", frame.mbz);
    self.stream.write_all(&frame.mbz).await?;
    
    self.stream.flush().await?;
    info!("Finished sending server greeting frame");

    Ok(())
  }

  /// Reads the server greeting frame from the server
  pub async fn read_server_greeting(&self) -> Result<Self, RfcError> {
    todo!();
  }

  /// Sends the setup response frame to the server.
  pub async fn send_setup_response(&mut self) -> Result<(), std::io::Error> {
    let frame = SetupResponseFrame::with_mode(self.mode);
    
    info!("Sending setup response frame");

    debug!("SetupResponseFrame [mode]: {:?}", self.mode);
    self.stream.write_all(&(self.mode as u32).to_be_bytes()).await?;

    debug!("SetupResponseFrame [key_id]: {:?}", frame.key_id);
    self.stream.write_all(&frame.key_id).await?;

    debug!("SetupResponseFrame [token]: {:?}", frame.token);
    self.stream.write_all(&frame.token).await?;

    debug!("SetupResponseFrame []: {:?}", frame.client_iv);
    self.stream.write_all(&frame.client_iv).await?;

    self.stream.flush().await?;
    info!("Finished sending setup response frame");
    
    Ok(())
  }

  /// Reads the setup response frame from the client.
  pub async fn read_setup_response(&self) -> Result<Self, RfcError> {
    todo!();
  }

  /// 
  pub async fn send_server_start(&self) -> Result<Self, RfcError> {
    todo!();
  }

  pub fn read_server_start(&self) -> Result<Self, RfcError> {
    todo!();
  }

  pub fn send_request_session(&self) -> Result<Self, RfcError> {
    todo!();
  }

  pub fn read_request_session(&self) -> Result<Self, RfcError> {
    todo!();
  }

  pub fn send_accept_session(&self) -> Result<Self, RfcError> {
    todo!();
  }

  pub fn receive_accept_session(&self) -> Result<Self, RfcError> {
    todo!();
  }

  pub fn send_start_session() -> Result<Self, RfcError> {
    todo!();
  }

  pub fn receive_start_session() -> Result<Self, RfcError> {
    todo!();
  }

  pub fn send_start_ack() -> Result<Self, RfcError> {
    todo!();
  }

  pub fn receive_start_ack() -> Result<Self, RfcError> {
    todo!();
  }

  pub fn send_stop_sessions() -> Result<Self, RfcError> {
    todo!();
  }

  pub fn receive_stop_sessions() -> Result<Self, RfcError> {
    todo!();
  }
}
