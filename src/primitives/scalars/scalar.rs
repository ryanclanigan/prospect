use super::float_scalar::FloatScalar;
use super::string_scalar::StringScalar;

/// Represents a scalar value of some sort
pub trait BaseScalar<T> {
    fn of(value: T) -> Self;
    fn to_value(&self) -> T;
}

#[derive(Clone, Debug)]
pub enum Scalar {
    String(StringScalar),
    Float(FloatScalar),
}
