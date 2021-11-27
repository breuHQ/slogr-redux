//! Common traits and utilites

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

/// Generate a random sequence of characters given the length
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
