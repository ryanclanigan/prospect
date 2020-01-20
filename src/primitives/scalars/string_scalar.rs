use super::scalar;

/// A scalar whose value is a string
#[derive(Debug, Clone)]
pub struct StringScalar {
  value: String,
}

impl scalar::Scalar<String> for StringScalar {
  fn of(value: &String) -> Self {
    StringScalar {
      value: value.clone(),
    }
  }

  fn to_value(&self) -> String {
    self.value.clone()
  }
}

#[cfg(test)]
mod test {
  use crate::primitives::scalars::scalar::Scalar;

  extern crate proptest;

  use super::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn test_of_and_value(s in ".*") {
      assert_eq!(s, StringScalar::of(&s).to_value());
    }
  }
}
