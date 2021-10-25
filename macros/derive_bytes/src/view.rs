use derive_bytes::ToBytes;

#[repr(u32)]
pub enum ServerGreetingMode {
  Unavailable = 0,
  Unauthenticated = 1,
  Authenticated = 2,
  Encrypted = 4,
}

#[repr(packed)]
#[derive(ToBytes)]
pub struct ServerGreetingFrame {
  pub unused: [u8; 12],

  // pub mode: ServerGreetingMode,
  pub challenge: [u8; 16],

  pub salt: [u8; 16],
  pub count: u32,
  pub mbz: [u8; 12],
}

fn main() {}
