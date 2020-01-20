use std::error::Error;

pub trait BaseOperation<T> {
  fn apply(&self, value: &T) -> Result<T, &'static str>;
}
