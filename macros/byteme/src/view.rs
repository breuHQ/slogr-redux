use byteme::ByteMe;

#[derive(Copy, Clone)]
pub enum ServerGreetingMode {
  Unavailable = 0,
  Unauthenticated = 1,
  Authenticated = 2,
  Encrypted = 4,
}

#[derive(ByteMe)]
pub struct ServerGreetingFrame {
  pub unused: [u8; 12],

  #[byte_me(u32)]
  pub mode: ServerGreetingMode,
  pub challenge: [u8; 16],

  pub salt: [u8; 16],
  pub count: u16,
  pub mbz: [u8; 12],
}

fn main() {}
