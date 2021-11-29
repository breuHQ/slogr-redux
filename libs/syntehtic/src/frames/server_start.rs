use super::Accept;
use crate::ntp::NtpTimestamp;
use byteme::ByteMe;
/// The server MUST respond with the following Server-Start message:
/// ```text
///    0                   1                   2                   3
///    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                                                               |
///    |                         MBZ (15 octets)                       |
///    |                                                               |
///    |                                               +-+-+-+-+-+-+-+-+
///    |                                               |   Accept      |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                                                               |
///    |                     Server-IV (16 octets)                     |
///    |                                                               |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                     Start-Time (Timestamp)                    |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                         MBZ (8 octets)                        |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Debug, Clone, Copy, ByteMe)]
pub struct ServerStartFrame {
  /// The MBZ parts MUST be zero.  The client MUST ignore their value.  MBZ
  /// (MUST be zero) fields here and after have the same semantics: the
  /// party that sends the message MUST set the field so that all bits are
  /// equal to zero; the party that interprets the message MUST ignore the
  /// value.  (This way, the field could be used for future extensions.)
  /// Server-IV is generated randomly by the server.  In unauthenticated
  /// mode, Server-IV is unused.
  pub mbz_1: [u8; 15],
  /// The Accept field indicates the server's willingness to continue
  /// communication.  A zero value in the Accept field means that the
  /// server accepts the authentication and is willing to conduct further
  /// transactions.  Non-zero values indicate that the server does not
  /// accept the authentication or, for some other reason, is not willing
  /// to conduct further transactions in this OWAMP-Control session.  The
  /// full list of available Accept values is described in Section 3.3,
  /// "Values of the Accept Field".
  ///
  /// /// If a negative (non-zero) response is sent, the server MAY (and the
  /// client SHOULD) close the connection after this message.
  #[byte_me(u8)]
  pub accept: Accept,
  /// Server-IV is generated randomly by the server.  In unauthenticated
  /// mode, Server-IV is unused.
  pub server_iv: [u8; 16],
  /// Start-Time is a timestamp representing the time when the current
  /// instantiation of the server started operating.  (For example, in a
  /// multi-user general purpose operating system, it could be the time
  /// when the server process was started.)  If Accept is non-zero, Start-
  /// Time SHOULD be set so that all of its bits are zeros.  In
  /// authenticated and encrypted modes, Start-Time is encrypted as
  /// described in Section 3.4, "OWAMP-Control Commands", unless Accept is
  /// non-zero.  (Authenticated and encrypted mode cannot be entered unless
  /// the control connection can be initialized.)
  /// Timestamp format is described in Section 4.1.2.  The same
  /// instantiation of the server SHOULD report the same exact Start-Time
  /// value to each client in each session.
  ///
  /// ## Summary
  /// The timpestamp is a 64-bit unsigned integer representing the number of
  /// seconds since the PRIME epoch (00:00:00 UTC, January 1, 1900). The first
  /// 32 bits represent the number of seconds since the PRIME epoch (00:00:00
  /// UTC, January 1, 1900). The last 32 bits represent the fractions. More
  /// information on [wikipedia](https://en.wikipedia.org/wiki/Network_Time_Protocol).
  ///
  pub start_time: u64,
  /// same as [`mbz1`]
  pub mbz_2: [u8; 8],
}

impl ServerStartFrame {
  /// Get the ServerStartFrame for unauthenticated mode
  pub fn unauthenticated(accept: Accept) -> Self {
    let start_time: u64 = u64::from(NtpTimestamp::now());
    Self {
      mbz_1: [0; 15],
      accept,
      server_iv: [0; 16],
      start_time,
      mbz_2: [0; 8],
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_into() {}

  #[test]
  fn test_from() {}
}
