use super::scalar;

/// A scalar whose value is a string
#[derive(Debug, Clone)]
pub struct FloatScalar {
  value: f64,
}

impl scalar::BaseScalar<f64> for FloatScalar {
  fn of(value: f64) -> Self {
    FloatScalar { value }
  }

  fn to_value(&self) -> f64 {
    self.value
  }
}

#[cfg(test)]
mod test {
  extern crate rand;

  use super::*;
  use crate::primitives::scalars::scalar::BaseScalar;
  use rand::Rng;
  use std::f64::{MAX, MIN};

  #[test]
  fn test_of_and_value() {
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
      let n: f64 = rng.gen_range(MIN, MAX);
      assert_eq!(n, FloatScalar::of(n).to_value());
    }
  }
}
