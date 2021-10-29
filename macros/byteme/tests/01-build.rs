use byteme::ByteMe;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

  let unused = bytes[0 .. 12].to_vec();
  
  let mode: [u8; 4] = bytes[12 .. 16].to_vec().as_slice().try_into().unwrap();
  let mode = u32::from_be_bytes(mode);
  let mode = match mode {
    0 => Mode::Unavailable,
    1 => Mode::Unauthenticated,
    2 => Mode::Authenticated,
    4 => Mode::Encrypted,
    _ => panic!("Unknown mode"),
  };

  let challenge = bytes[16 .. 32].to_vec();
  let salt = bytes[32 .. 48].to_vec();
  let count: [u8; 4] = bytes[48 .. 52].to_vec().as_slice().try_into().unwrap();
  let count = u32::from_be_bytes(count);
  let mbz = bytes[52 .. 64].to_vec();

  assert_eq!(bytes.len(), 64);
  assert_eq!(unused, frame.unused);
  assert_eq!(mode, frame.mode);
  assert_eq!(challenge, frame.challenge);
  assert_eq!(salt, frame.salt);
  assert_eq!(count, frame.count);
  assert_eq!(mbz, frame.mbz);
}
