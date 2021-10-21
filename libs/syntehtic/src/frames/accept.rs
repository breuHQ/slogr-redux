use serde::{Deserialize, Serialize};

/// The full set of valid Server-Start accept value as described in RFC.   
///
/// Accept values are used throughout the TWAMP-Control protocol to
/// communicate the server response to client requests.  The full set of
/// valid Accept field values are as follows:
///
///    - 0    OK.
///    - 1    Failure, reason unspecified (catch-all).
///    - 2    Internal error.
///    - 3    Some aspect of request is not supported.
///    - 4    Cannot perform request due to permanent resource limitations.
///    - 5    Cannot perform request due to temporary resource limitations.
///
///   All other values are reserved.  The sender of the message MAY use the
///  value of 1 for all non-zero Accept values.  A message sender SHOULD
///  use the correct Accept value if it is going to use other values.  The
///  message receiver MUST interpret all values of Accept other than these
///  reserved values as 1.  This way, other values are available for
///  future extensions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Accept {
  /// Ok
  Ok = 0,
  /// Failure, reason unspecified (catch-all).
  Failure = 1,
  /// Internal Error
  InternalError = 2,
  /// Some aspect of request is not supported
  RequestNotSupported = 3,
  /// Cannot perform request due to permanent resource limitations
  PermanentResourceLimitation = 4,
  /// Cannot perform request due to temporary resource limitations
  TemporaryResourceLimitation = 5,
}
