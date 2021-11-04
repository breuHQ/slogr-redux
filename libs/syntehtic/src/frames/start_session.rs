//!    Having requested one or more test sessions and received affirmative
//! Accept-Session responses, an OWAMP client MAY start the execution of
//! the requested test sessions by sending a Start-Sessions message to
//! the server.
//! The format of this message is as follows:
//!
//!   0                   1                   2                   3
//!   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//!   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!   |      2        |                                               |
//!   +-+-+-+-+-+-+-+-+                                               |
//!   |                        MBZ (15 octets)                        |
//!   |                                                               |
//!   |                                                               |
//!   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!   |                                                               |
//!   |                       HMAC (16 octets)                        |
//!   |                                                               |
//!   |                                                               |
//!   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

use byteme::ByteMe;

/// Instructions for the server to start the test sessions.
#[derive(Debug, PartialEq, ByteMe, Eq)]
pub struct StartSessionFrame {
  /// ID of the session to start, hard coded to 2
  pub id: u8,
  /// MBZ, hard coded to 15 octets
  pub mbz: [u8; 15],
  /// HMAC, hard coded to 16 octets
  pub hmac: [u8; 16],
}
