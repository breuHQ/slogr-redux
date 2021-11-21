//!    The procedure and guidelines for stopping test sessions is similar to
//!    that defined in Section 3.8 of OWAMP [RFC4656].  The Stop-Sessions
//!    command can only be issued by the Control-Client.  The message MUST
//!    NOT contain any session description records or skip ranges.  The
//!    message is terminated with a single block HMAC to complete the Stop-
//!    Sessions command.  Since the TWAMP Stop-Sessions command does not
//!    convey SIDs, it applies to all sessions previously requested and
//!    started with a Start-Sessions command.
//!    Thus, the TWAMP Stop-Sessions command is constructed as follows:
//!
//!     0                   1                   2                   3
//!     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |      3        |    Accept     |              MBZ              |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |                      Number of Sessions                       |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |                        MBZ (8 octets)                         |
//!    |                                                               |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |                                                               |
//!    |                       HMAC (16 octets)                        |
//!    |                                                               |
//!    |                                                               |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!
//!    Above, the Command Number in the first octet (3) indicates that this
//!    is the Stop-Sessions command.
//!    Non-zero Accept values indicate a failure of some sort.  Zero values
//!    indicate normal (but possibly premature) completion.  The full list
//!    of available Accept values is described in Section 3.3 of [RFC4656],
//!    "Values of the Accept Field".
//!    If Accept has a non-zero value, results of all TWAMP-Test sessions
//!    spawned by this TWAMP-Control session SHOULD be considered invalid.
//!    If the Accept-Session message was not transmitted at all (for
//!    whatever reason, including failure of the TCP connection used for
//!    TWAMP-Control), the results of all TWAMP-Test sessions spawned by
//!    this TWAMP-Control session MAY be considered invalid.
//!    Number of Sessions indicates the number of sessions that the
//!    Control-Client intends to stop.
//!    Number of Sessions MUST contain the number of send sessions started
//!    by the Control-Client that have not been previously terminated by a
//!    Stop-Sessions command (i.e., the Control-Client MUST account for each
//!    accepted Request-Session).  If the Stop-Sessions message does not
//!    account for exactly the number of sessions in progress, then it is to
//!    be considered invalid, the TWAMP-Control connection SHOULD be closed,
//!    and any results obtained considered invalid.
//!    Upon receipt of a TWAMP-Control Stop-Sessions command, the Session-
//!    Reflector MUST discard any TWAMP-Test packets that arrive at the
//!    current time plus the Timeout (in the Request-TW-Session command).
use byteme::ByteMe;

use crate::frames::Accept;

/// The Stop Session Frame
#[derive(Debug, PartialEq, Eq, Clone, Copy, ByteMe)]
pub struct StopSessionFrame {
  /// command number. The value of this field MUST be 3.
  pub command_number: u8,
  /// Accept value.
  #[byte_me(u8)]
  pub accept: Accept,
  /// MBZ.
  pub mbz: u16,
  /// Number of Sessions.
  pub number_of_sessions: u32,
  /// MBZ.
  pub mbz_2: [u8; 8],
  /// HMAC.
  pub hmac: [u8; 16],
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_into() {}

  #[test]
  fn test_from() {}
}
