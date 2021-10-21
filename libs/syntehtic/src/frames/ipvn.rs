use serde::{Deserialize, Serialize};

/// IP versions as byte
#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IpVn {
  /// Represented ip version 4
  V4 = 4,
  /// Represents IP version 6
  V6 = 6,
}
