//! To each Request-Session message, an OWAMP server MUST respond with an
//! Accept-Session message:
//!   0                   1                   2                   3
//!   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |    Accept     |  MBZ          |            Port               |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-|
//!  |                                                               |
//!  |                        SID (16 octets)                        |
//!  |                                                               |
//!  |                                                               |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                                                               |
//!  |                        MBZ (12 octets)                        |
//!  |                                                               |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                                                               |
//!  |                       HMAC (16 octets)                        |
//!  |                                                               |
//!  |                                                               |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  In this message, zero in the Accept field means that the server is
//!  willing to conduct the session.  A non-zero value indicates rejection
//!  of the request.  The full list of available Accept values is
//!  described in Section 3.3, "Values of the Accept Field".
//!  If the server rejects a Request-Session message, it SHOULD not close
//!  the TCP connection.  The client MAY close it if it receives a
//!  negative response to the Request-Session message.
//!  The meaning of Port in the response depends on the values of Conf-
//!  Sender and Conf-Receiver in the query that solicited the response.
//!  If both were set, the Port field is unused.  If only Conf-Sender was
//!  set, Port is the port from which to expect OWAMP-Test packets.  If
//!  only Conf-Receiver was set, Port is the port to which OWAMP-Test
//!  packets are sent.
//!  If only Conf-Sender was set, the SID field in the response is unused.
//!  Otherwise, SID is a unique server-generated session identifier.  It
//!  can be used later as handle to fetch the results of a session.
//!  SIDs SHOULD be constructed by concatenation of the 4-octet IPv4 IP
//!  number belonging to the generating machine, an 8-octet timestamp, and
//!  a 4-octet random value.  To reduce the probability of collisions, if
//! the generating machine has any IPv4 addresses (with the exception of
//!  loopback), one of them SHOULD be used for SID generation, even if all
//!  communication is IPv6-based.  If it has no IPv4 addresses at all, the
//!  last four octets of an IPv6 address MAY be used instead.  Note that
//!  SID is always chosen by the receiver.  If truly random values are not
//!  available, it is important that the SID be made unpredictable, as
//!  knowledge of the SID might be used for access control.

use byteme::ByteMe;

use crate::frames::Accept;

/// Respond to a Request-Session message with an Accept-Session message.
#[derive(Debug, PartialEq, Eq, ByteMe)]
pub struct AcceptSessionFrame {
  #[byte_me(u8)]
  /// The Accept field.
  pub accept: Accept,
  /// MBZ field.
  pub mbz: u8,
  /// The port to which to send Test packets.
  pub port: u16,
  /// The SID.
  pub sid: [u8; 16],
  /// MBZ field.
  pub mbz2: [u8; 12],
  /// The HMAC.
  pub hmac: [u8; 16],
}
