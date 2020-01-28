use super::scalar;

/// A scalar whose value is a string
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringScalar {
    value: String,
}

impl scalar::BaseScalar<String> for StringScalar {
    fn of(value: String) -> Self {
        StringScalar { value }
    }

    fn to_value(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::scalars::scalar::BaseScalar;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn test_of_and_value(s in ".*") {
        assert_eq!(s.clone(), StringScalar::of(s).to_value());
      }
    }
}
