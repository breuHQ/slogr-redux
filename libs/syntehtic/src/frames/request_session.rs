use super::IpVn;
use serde::{Deserialize, Serialize};

/// Test session creation follows the same procedure as defined in
/// Section 3.5 of OWAMP [RFC4656].  The Request-TW-Session command is
/// based on the OWAMP Request-Session command, and uses the message
/// format as described in Section 3.5 of OWAMP, but without the Schedule
/// Slot Descriptions field(s) and uses only one HMAC.  The description
/// of the Request-TW-Session format follows.
///
/// The Start Time is as defined in OWAMP [RFC4656].
///
/// The Timeout is interpreted differently from the definition in OWAMP
/// [RFC4656].  In TWAMP, Timeout is the interval that the Session-
/// Reflector MUST wait after receiving a Stop-Sessions message.  In case
/// there are test packets still in transit, the Session-Reflector MUST
/// reflect them if they arrive within the Timeout interval following the
/// reception of the Stop-Sessions message.  The Session-Reflector MUST
/// NOT reflect packets that are received beyond the timeout.
/// Type-P descriptor is as defined in OWAMP [RFC4656].  The only
/// capability of this field is to set the Differentiated Services Code
/// Point (DSCP) as defined in [RFC2474].  The same value of DSCP MUST be
/// used in test packets reflected by the Session-Reflector.
///
/// The Session-Reflector MUST respond to each Request-TW-Session command
/// with an Accept-Session message as defined in OWAMP [RFC4656].  When
/// the Accept field = 0, the Port field confirms (repeats) the port to
/// which TWAMP-Test packets are sent by the Session-Sender toward the
/// Session-Reflector.  In other words, the Port field indicates the port
/// number where the Session-Reflector expects to receive packets from
/// the Session-Sender.
///
/// ## The format of Request-Session message is as follows:
///
/// ```text
///     0                   1                   2                   3
///     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |      1        |  MBZ  | IPVN  |  Conf-Sender  | Conf-Receiver |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                  Number of Schedule Slots                     |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                      Number of Packets                        |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |          Sender Port          |         Receiver Port         |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                        Sender Address                         |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                                                               |
///    |           Sender Address (cont.) or MBZ (12 octets)           |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                        Receiver Address                       |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                                                               |
///    |           Receiver Address (cont.) or MBZ (12 octets)         |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                                                               |
///    |                        SID (16 octets)                        |
///    |                                                               |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                         Padding Length                        |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                           Start Time                          |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                       Timeout, (8 octets)                     |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                       Type-P Descriptor                       |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                         MBZ (8 octets)                        |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                                                               |
///    |                       HMAC (16 octets)                        |
///    |                                                               |
///    |                                                               |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[repr(packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RequestSessionFrame {
  /// If a TWAMP Server receives an unexpected Command Number, it MUST
  /// respond with the Accept field set to 3 (meaning "Some aspect of
  /// request is not supported") in the Accept-Session message.  Command
  /// Numbers that are Forbidden (and possibly numbers that are Reserved)
  /// are unexpected.
  pub command_number: u8,
  /// mbz
  pub mbz_1: u8,
  ///  IPVN is the IP version numbers for Sender and Receiver.  When the IP
  ///  version number is 4, 12 octets follow the 4-octet IPv4 address stored
  ///  in Sender Address and Receiver Address.  These octets MUST be set to
  ///  zero by the client and MUST be ignored by the server.  Currently
  ///  meaningful IPVN values are 4 and 6.
  pub ipvn: IpVn,
  /// In OWAMP, the Conf-Sender field is set to 1 when the Request-Session
  /// message describes a task where the Server will configure a one-way
  /// test packet sender.  Likewise, the Conf-Receiver field is set to 1
  /// when the message describes the configuration for a Session-Receiver.
  /// In TWAMP, both endpoints send and receive test packets, with the
  /// Session-Sender first sending and then receiving test packets,
  /// complimented by the Session-Reflector first receiving and then
  /// sending.
  ///
  /// Both the Conf-Sender field and Conf-Receiver field MUST be set to 0
  /// since the Session-Reflector will both receive and send packets, and
  /// the roles are established according to which host initiates the TCP
  /// connection for control.  The Server MUST interpret any non-zero value
  /// as an improperly formatted command, and MUST respond with the Accept
  /// field set to 3 (meaning "Some aspect of request is not supported") in
  /// the Accept-Session message.
  ///  Conf-Sender and Conf-Receiver MUST be set to 0 or 1 by the client.
  ///  The server MUST interpret any non-zero value as 1.  If the value is
  ///  1, the server is being asked to configure the corresponding agent
  ///  (sender or receiver).  In this case, the corresponding Port value
  ///  SHOULD be disregarded by the server.  At least one of Conf-Sender and
  ///  Conf-Receiver MUST be 1.  (Both can be set, in which case the server
  ///  is being asked to perform a session between two hosts it can
  ///  configure.)
  ///  If Conf-Sender is not set, Sender Port is the UDP port from which
  ///  OWAMP-Test packets will be sent.  If Conf-Receiver is not set,
  ///  Receiver Port is the UDP port OWAMP-Test to which packets are
  ///  requested to be sent.
  pub conf_sender: u8,
  /// Same as `conf_sender`
  pub conf_receiver: u8,
  ///  Number of Packets is the number of active measurement packets to be
  ///  sent during this OWAMP-Test session (note that either the server or
  ///  the client can abort the session early).
  pub number_of_packets: u32,
  /// The Session-Reflector in TWAMP does not process incoming test packets
  /// for performance metrics and consequently does not need to know the
  /// number of incoming packets and their timing schedule.  Consequently
  /// the Number of Scheduled Slots and Number of Packets MUST be set to 0.
  /// The Sender Port is the UDP port from which TWAMP-Test packets will be
  /// sent and the port to which TWAMP-Test packets will be sent by the
  /// Session-Reflector (the Session-Sender will use the same UDP port to
  /// send and receive packets).  The Receiver Port is the desired UDP port
  /// to which TWAMP-Test packets will be sent by the Session-Sender (the
  /// port where the Session-Reflector is asked to receive test packets).
  /// The Receiver Port is also the UDP port from which TWAMP-Test packets
  /// will be sent by the Session-Reflector (the Session-Reflector will use
  /// the same UDP port to send and receive packets).
  ///
  /// When the requested Receiver Port is not available (e.g., port in
  /// use), the Server at the Session-Reflector MAY suggest an alternate
  /// and available port for this session in the Port field.  The Session-
  /// Sender either accepts the alternate port, or composes a new Session-
  /// Request message with suitable parameters.  Otherwise, the Server at
  /// the Control-Client uses the Accept field to convey other forms of
  /// session rejection or failure and MUST NOT suggest an alternate port;
  /// in this case, the Port field MUST be set to zero.
  pub sender_port: u16,
  /// See `sender_port`
  pub receiver_port: u16,
  ///  The Sender Address and Receiver Address fields contain, respectively,
  ///  the sender and receiver addresses of the end points of the Internet
  ///  path over which an OWAMP test session is requested.
  pub sender_address: u32,
  /// if `ipvn` is v6, mbz is used to fill data.
  pub sender_mbz: [u8; 12],
  /// See `sender_address`.
  pub receiver_address: u32,
  /// See `sender_mbz`
  pub receiver_mbz: [u8; 12],
  ///  SID is the session identifier.  It can be used in later sessions as
  ///  an argument for the Fetch-Session command.  It is meaningful only if
  ///  Conf-Receiver is 0.  This way, the SID is always generated by the
  ///  receiving side.  See the end of the section for information on how
  ///  the SID is generated.
  /// The Session Identifier (SID) is as defined in OWAMP [RFC4656].  Since
  /// the SID is always generated by the receiving side, the Server
  /// determines the SID, and the SID in the Request-TW-Session message
  /// MUST be set to 0.
  pub sid: [u8; 16],
  ///  Padding length is the number of octets to be appended to the normal
  ///  OWAMP-Test packet (see more on padding in discussion of OWAMP-Test).
  pub padding_length: u32,
  ///  Start Time is the time when the session is to be started (but not
  ///  before Start-Sessions command is issued).  This timestamp is in the
  ///  same format as OWAMP-Test timestamps.
  pub start_time: u32,
  ///  Timeout (or a loss threshold) is an interval of time (expressed as a
  ///  timestamp).  A packet belonging to the test session that is being set
  ///  up by the current Request-Session command will be considered lost if
  ///  it is not received during Timeout seconds after it is sent.
  pub time_out: u32,
  ///  Type-P Descriptor covers only a subset of (very large) Type-P space.
  ///  If the first two bits of the Type-P Descriptor are 00, then the
  ///  subsequent six bits specify the requested Differentiated Services
  ///  Codepoint (DSCP) value of sent OWAMP-Test packets, as defined in
  ///  [RFC2474].  If the first two bits of Type-P descriptor are 01, then
  ///  the subsequent 16 bits specify the requested PHB Identification Code
  ///  (PHB ID), as defined in [RFC2836].
  ///
  ///  Therefore, the value of all zeros specifies the default best-effort
  ///  service.
  ///
  ///  If Conf-Sender is set, the Type-P Descriptor is to be used to
  ///  configure the sender to send packets according to its value.  If
  ///  Conf-Sender is not set, the Type-P Descriptor is a declaration of how
  ///  the sender will be configured.
  ///
  ///  If Conf-Sender is set and the server does not recognize the Type-P
  ///  Descriptor, or it cannot or does not wish to set the corresponding
  ///  attributes on OWAMP-Test packets, it SHOULD reject the session
  ///  request.  If Conf-Sender is not set, the server SHOULD accept or
  ///  reject the session, paying no attention to the value of the Type-P
  ///  Descriptor.
  pub type_p_descriptor: u32,
  /// MBZ
  pub mbz: u16,
  /// Since there are no Schedule Slot Descriptions, the Request-TW-Session
  /// message is completed by MBZ (Must Be Zero) and HMAC fields.  This
  /// completes one logical message, referred to as the Request-TW-Session
  /// command.
  pub hmac: [u8; 16],
}
