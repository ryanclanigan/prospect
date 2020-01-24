use super::float_scalar::FloatScalar;
use super::string_scalar::StringScalar;
use std::fmt;

/// Represents a scalar value of some sort
pub trait BaseScalar<T> {
    fn of(value: T) -> Self;
    fn to_value(&self) -> T;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Scalar {
    String(StringScalar),
    Float(FloatScalar),
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match &self {
            Scalar::String(s) => format!("String Scalar: {}.", s.to_value()),
            Scalar::Float(f) => format!("Float Scalar: {}.", f.to_value()),
        };
        write!(f, "{}", message)
    }
}
