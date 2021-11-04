//! The server MUST respond with an Start-Ack message (which SHOULD be
//! sent as quickly as possible).  Start-Ack messages have the following
//! format:
//!   0                   1                   2                   3
//!   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//!   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!   |     Accept    |                                               |
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
//! If Accept is non-zero, the Start-Sessions request was rejected; zero
//! means that the command was accepted.  The full list of available
//! Accept values is described in Section 3.3, "Values of the Accept
//! Field".  The server MAY, and the client SHOULD, close the connection
//! in the case of a rejection.
//! The server SHOULD start all OWAMP-Test streams immediately after it
//! sends the response or immediately after their specified start times,
//! whichever is later.  If the client represents a Sender, the client
//! SHOULD start its OWAMP-Test streams immediately after it sees the
//! Start-Ack response from the Server (if the Start-Sessions command was
//! accepted) or immediately after their specified start times, whichever
//! is later.  See more on OWAMP-Test sender behavior in a separate
//! section below.

use byteme::ByteMe;

use crate::frames::Accept;

/// Response to a Start-Sessions command.
#[derive(Debug, PartialEq, Eq, ByteMe, Clone, Copy)]
pub struct StartAckFrame {
  #[byte_me(u8)]
  /// The value of the Accept field.
  pub accept: Accept,
  /// The value of the MBZ field.
  pub mbz: [u8; 15],
  /// The value of the HMAC field.
  pub hmac: [u8; 16],
}
