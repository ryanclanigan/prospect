use std::convert::TryFrom;
use std::fmt;
use std::ops;

#[derive(Copy, Debug, Clone)]
pub struct F64 {
    value: f64,
}

impl F64 {
    pub fn of(value: f64) -> Self {
        F64 { value }
    }

    pub fn to_value(self) -> f64 {
        self.value
    }
}

impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        if self.value.is_nan() || other.value.is_nan() {
            return false;
        }
        if self.value == other.value {
            return true;
        }
        false
    }
}

impl Eq for F64 {}

impl ops::Add<F64> for F64 {
    type Output = F64;

    fn add(self, _rhs: F64) -> F64 {
        if self.value.is_nan() {
            return F64::of(std::f64::NAN);
        }
        if _rhs.to_value().is_nan() {
            return F64::of(std::f64::NAN);
        }
        F64::of(self.value + _rhs.to_value())
    }
}

impl ops::Sub for F64 {
    type Output = F64;

    fn sub(self, _rhs: F64) -> F64 {
        if self.value.is_nan() {
            return F64::of(std::f64::NAN);
        }
        if _rhs.to_value().is_nan() {
            return F64::of(std::f64::NAN);
        }
        F64::of(self.value - _rhs.to_value())
    }
}

impl fmt::Display for F64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<&String> for F64 {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.parse() {
            Ok(f) => Ok(F64::of(f)),
            Err(_) => Err("Could not convert string to F64"),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate proptest;

    use super::*;
    use proptest::prelude::*;
    use std::f64::MAX;

    proptest! {
      #[test]
      fn test_ops(float1 in 0f64..MAX/2f64, float2 in 0f64..MAX/2f64) {
        let f1 = F64::of(float1);
        let f2 = F64::of(float2);
        assert_eq!(float1 - float2, (f1 - f2).to_value());
        assert_eq!(float1 + float2, (f1 + f2).to_value());
        assert_eq!(float2 - float1, (f2 - f1).to_value());
      }
    }
}
