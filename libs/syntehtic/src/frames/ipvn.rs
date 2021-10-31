use num_derive::FromPrimitive;
/// IP versions as byte
#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum IpVn {
  /// Represented ip version 4
  V4 = 4,
  /// Represents IP version 6
  V6 = 6,
}
