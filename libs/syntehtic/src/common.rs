//! Common utilities

/// Both server and client needs to know the key_iv, secret map to encrypt/decypt data
/// TODO: this is to make progress, come up with a logic to share keys securely
pub const KEY_ID: &str = "EIKDY9tK0E5G61GsnSgGjm4gB4FJ9lvpklkI538QgmHEudQQJEMwMU8qvxX1X2O4JXypv4zVCAg8HsJE";
/// Trait to Implement `reply()` method
pub trait Reply {
  /// type of Response
  type Response;
  /// given the frame, formulte a reply
  fn reply(&self) -> Self::Response;
}

/// Trait to implement `is_valid` method
pub trait IsValid {
  /// checks if the struct is valid
  fn is_valid(&self) -> bool;
}

/// Quickly implement a life time for a struct intended to be shared between threads.
pub trait SendSyncStatic: Send + Sync + 'static {}

/// Generate a random sequence of characters given the length. Under the hood, it uses `thread_rng` which is crypto
/// secure as per the [documentation](https://docs.rs/rand/0.8.4/rand/rngs/struct.ThreadRng.html)
pub fn generate_random_sequence(len: usize) -> String {
  use rand::distributions::Alphanumeric;
  use rand::{thread_rng, Rng};

  let seq: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect();

  seq
}
