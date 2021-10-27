use serde::{Deserialize, Serialize};

/// The value of the Modes field sent by the server is the bit-wise OR of the
/// mode values that it is willing to support during this session. Thus, the
/// last three bits of the Modes 32-bit value are used.  The first
/// 29 bits MUST be zero.  A client MUST ignore the values in the first 29 bits
/// of the Modes value.  
///
/// > This way, the bits are available for future protocol extensions.
/// > This is the only intended extension mechanism.)
///
/// If the Modes value is zero, the server does not wish to communicate
/// with the client and MAY close the connection immediately.  The client
/// SHOULD close the connection if it receives a greeting with Modes
/// equal to zero.  The client MAY close the connection if the client's
/// desired mode is unavailable.
#[repr(u32)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
  /// 0: The server is not willing to accept any more sessions.
  Unavailable = 0,
  /// 1: The serve is available in unautheticated mode.
  Unauthenticated = 1,
  /// 2: The serve is available in authenticated mode only.
  Authenticated = 2,
  /// 4: The server is available in authenticated and encrypted mode only.
  Encrypted = 4,
}
