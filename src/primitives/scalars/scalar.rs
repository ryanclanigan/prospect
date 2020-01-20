/// Represents a scalar value of some sort
pub trait Scalar<T> {
  fn of(value: &T) -> Self;
  fn to_value(&self) -> T;
}
