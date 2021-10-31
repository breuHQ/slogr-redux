use byteme::ByteMe;
pub use num_derive::FromPrimitive;
pub use num_traits::FromPrimitive;


#[derive(Debug, FromPrimitive, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
  Unavailable = 0,
  Unauthenticated = 1,
  Authenticated = 2,
  Encrypted = 4,
}

#[derive(ByteMe)]
pub struct FrameOne {
  pub unused: [u8; 12],
  #[byte_me(u32)]
  pub mode: Mode,
  pub challenge: [u8; 16],
  pub salt: [u8; 16],
  pub count: u32,
  pub mbz: [u8; 12],
}


fn main() {
  let frame = FrameOne {
    unused: [0; 12],
    mode: Mode::Unauthenticated,
    challenge: [0; 16],
    salt: [0; 16],
    count: 1024,
    mbz: [0; 12],
  };

  let bytes = frame.to_bytes();

  let unused: [u8; 12] = bytes[0 .. 12].try_into().unwrap();
  
  let mode: [u8; 4] = bytes[12 .. 16].try_into().unwrap();
  let mode = u32::from_be_bytes(mode);
  let mode = Mode::from_u32(mode).unwrap();

  let challenge: [u8; 16] = bytes[16 .. 32].try_into().unwrap();
  let salt: [u8; 16] = bytes[32 .. 48].try_into().unwrap();
  let count: [u8; 4] = bytes[48 .. 52].try_into().unwrap();
  let count = u32::from_be_bytes(count);
  let mbz: [u8; 12] = bytes[52 .. 64].try_into().unwrap();

  assert_eq!(bytes.len(), 64);
  assert_eq!(unused, frame.unused);
  assert_eq!(mode, frame.mode);
  assert_eq!(challenge, frame.challenge);
  assert_eq!(salt, frame.salt);
  assert_eq!(count, frame.count);
  assert_eq!(mbz, frame.mbz);
}
