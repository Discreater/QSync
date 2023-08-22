pub trait WithError {
  type Item;
  fn with_err<F>(self, f: F)
  where
    F: FnOnce(Self::Item);
}

impl<T, E> WithError for Result<T, E> {
  type Item = E;
  fn with_err<F>(self, f: F)
  where
    F: FnOnce(Self::Item),
  {
    if let Err(e) = self {
      f(e);
    }
  }
}
