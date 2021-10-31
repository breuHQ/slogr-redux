use super::Mode;
use num_traits::FromPrimitive;
use byteme::ByteMe;

/// The client MUST respond with the following Set-Up-Response message:
///
/// ```text
///    0                   1                   2                   3
///    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |                             Mode                              |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |                                                               |
///   .                                                               .
///   .                       KeyID (80 octets)                       .
///   .                                                               .
///   |                                                               |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |                                                               |
///   .                                                               .
///   .                       Token (64 octets)                       .
///   .                                                               .
///   |                                                               |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |                                                               |
///   .                                                               .
///   .                     Client-IV (16 octets)                     .
///   .                                                               .
///   |                                                               |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Debug, Clone, Copy, ByteMe)]
pub struct SetupResponseFrame {
  /// Here Mode is the mode that the client chooses to use during this
  /// TWAMP-Control session.  It will also be used for all TWAMP-Test
  /// sessions started under control of this OWAMP-Control session.  In
  /// Mode, one or zero bits MUST be set within last three bits.  If it is
  /// one bit that is set within the last three bits, this bit MUST
  /// indicate a mode that the server agreed to use (i.e., the same bit
  /// MUST have been set by the server in the server greeting).  The first
  /// 29 bits of Mode MUST be zero.  A server MUST ignore the values of the
  /// first 29 bits.  If zero Mode bits are set by the client, the client
  /// indicates that it will not continue with the session; in this case,
  /// the client and the server SHOULD close the TCP connection associated
  /// with the OWAMP-Control session.
  #[byte_me(u32)]
  pub mode: Mode,
  /// In unauthenticated mode, KeyID, Token, and Client-IV are unused.
  /// Otherwise, KeyID is a UTF-8 string, up to 80 octets in length (if the
  /// string is shorter, it is padded with zero octets), that tells the
  /// server which shared secret the client wishes to use to authenticate
  /// or encrypt,
  pub key_id: [u8; 80],
  /// while Token is the concatenation of a 16-octet challenge,
  /// a 16-octet AES Session-key used for encryption, and a 32-octet HMAC-
  /// SHA1 Session-key used for authentication.  The token itself is
  /// encrypted using the AES (Advanced Encryption Standard) in
  /// Cipher Block Chaining (CBC). Encryption MUST be performed using an
  /// Initialization Vector (IV) of zero and a key derived from the shared
  /// secret associated with KeyID.  (Both the server and the client use
  /// the same mappings from KeyIDs to shared secrets.  The server, being
  /// prepared to conduct sessions with more than one client, uses KeyIDs
  /// to choose the appropriate secret key; a client would typically have
  /// different secret keys for different servers.  The situation is
  /// analogous to that with passwords.)
  ///
  /// The shared secret is a passphrase; it MUST not contain newlines.  The
  /// secret key is derived from the passphrase using a password-based key
  /// derivation function PBKDF2 (PKCS #5) RFC2898.  The PBKDF2 function
  /// requires several parameters: the PRF is HMAC-SHA1 RFC2104; the salt
  /// and count are as transmitted by the server.
  pub token: [u8; 64],
  /// AES Session-key, HMAC Session-key and Client-IV are generated
  /// randomly by the client.  AES Session-key and HMAC Session-key MUST be
  /// generated with sufficient entropy not to reduce the security of the
  /// underlying cipher RFC4086.  Client-IV merely needs to be unique
  /// (i.e., it MUST never be repeated for different sessions using the
  /// same secret key; a simple way to achieve that without the use of
  /// cumbersome state is to generate the Client-IV values using a
  /// cryptographically secure pseudo-random number source:  if this is
  /// done, the first repetition is unlikely to occur before 2^64 sessions
  /// with the same secret key are conducted).
  pub client_iv: [u8; 16],
}

impl SetupResponseFrame {
  /// Given the [`ServerGreetingMode`], We generate the setup response.
  /// TODO: complete for all cases
  pub fn with_mode(mode: Mode) -> Self {
    match mode {
      Mode::Authenticated => todo!(),
      Mode::Unauthenticated => Self {
        mode,
        key_id: [0; 80],
        token: [0; 64],
        client_iv: [0; 16],
      },
      Mode::Unavailable => todo!(),
      Mode::Encrypted => todo!(),
    }
  }
}
