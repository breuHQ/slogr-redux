/// Trait to Implement `reply()` method
pub trait Reply<T> {
  /// given the frame, formulate a reply
  fn reply(&self) -> T;
}

/// A common trait that can be shared across threads and all the properties live throughout the lifetime of the trait
pub trait SyncSendStatic: Send + Sync + 'static {}
