//! DateTime for NTP
use chrono::DateTime;
use chrono::Utc;
use chrono::{TimeZone, Timelike};

/// Delta from the Unix epoch.
const PRIME_EPOCH_DELTA: u64 = 2_208_988_800;

/// The byte layout of the NTP timestamp
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NtpTimestamp {
  /// 32 bit representation in seconds
  pub seconds: u32,
  /// 32 bit representation after seconds
  pub fraction: u32,
}

/// Basic implentation. Taken from
/// Converts a NTP timestamp to a DateTime
/// https://books.google.com.pk/books?id=x1w4EAAAQBAJ&pg=PA319&lpg=PA319&dq=impl+From%3CNtpTimeStamp%3E+for+DateTime%3CUtc%3E+rust&source=bl&ots=O0MEZZr9fH&sig=ACfU3U1KkmXgxHkjc44EQ1CL1ayo6EqwJA&hl=en&sa=X&ved=2ahUKEwjnzLDLk4f0AhVSRBoKHdkSBUIQ6AF6BAgZEAM#v=onepage&q&f=false
impl From<NtpTimestamp> for DateTime<Utc> {
  fn from(ntp_timestamp: NtpTimestamp) -> Self {
    let seconds = ntp_timestamp.seconds as i64 - PRIME_EPOCH_DELTA as i64;
    let mut nanos = ntp_timestamp.fraction as f64;
    nanos *= 1e9;
    nanos /= 2_f64.powi(32);

    Utc.timestamp(seconds, nanos as u32)
  }
}

/// Convert a DateTime to a NTP timestamp
impl From<DateTime<Utc>> for NtpTimestamp {
  fn from(dt: DateTime<Utc>) -> Self {
    let seconds = dt.timestamp() as i64 + PRIME_EPOCH_DELTA as i64;
    let fraction = dt.nanosecond() as f64;
    let fraction = (fraction * 2_f64.powi(32)) / 1e9;

    NtpTimestamp {
      seconds: seconds as u32,
      fraction: fraction as u32,
    }
  }
}

/// Converts the timestamp to u64
impl From<NtpTimestamp> for u64 {
  fn from(ntp_timestamp: NtpTimestamp) -> Self {
    let seconds = ntp_timestamp.seconds as u64;
    let fraction = ntp_timestamp.fraction as u64;
    seconds << 32 | fraction
  }
}

/// converts a u64 to a NtpTimestamp
impl From<u64> for NtpTimestamp {
  fn from(timestamp: u64) -> Self {
    let seconds = (timestamp >> 32) as u32;
    let fraction = timestamp as u32;

    NtpTimestamp { seconds, fraction }
  }
}

impl NtpTimestamp {
  /// Create an NTP timestamp from a DateTime
  pub fn new(seconds: u32, fraction: u32) -> Self {
    NtpTimestamp { seconds, fraction }
  }

  /// Get the current NTP timestamp
  pub fn now() -> Self {
    let now = Utc::now();
    NtpTimestamp::from(now)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ntp_to_utc() {
    let now = NtpTimestamp {
      seconds: 15_848_988,
      fraction: 19_800_911,
    };
    let utc: DateTime<Utc> = DateTime::from(now);
    let now_again = NtpTimestamp::from(utc);

    assert_ne!(now, now_again);
  }

  #[test]
  fn utc_to_ntp() {
    let now = Utc::now();
    let ntp_timestamp: NtpTimestamp = NtpTimestamp::from(now);
    let now_again = DateTime::from(ntp_timestamp);

    assert_ne!(now, now_again);
  }

  #[test]
  fn ntp_to_u64() {
    let now = NtpTimestamp::now();
    let number: u64 = u64::from(now);
    let now_again = NtpTimestamp::from(number);
    assert_eq!(now, now_again);
  }
}
