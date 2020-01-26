use super::scalars::scalar::Scalar;
use chrono::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Sample {
    pub time: DateTime<Utc>,
    pub value: Scalar,
}

impl Sample {
    pub fn is_keyed_before(&self, other: &Sample) -> bool {
        self.time < other.time
    }

    #[allow(dead_code)]
    pub fn is_keyed_at(&self, other: &Sample) -> bool {
        self.time == other.time
    }

    pub fn is_keyed_after(&self, other: &Sample) -> bool {
        self.time > other.time
    }

    #[allow(dead_code)]
    pub fn of(value: Scalar, time: DateTime<Utc>) -> Self {
        Sample { value, time }
    }
}

#[cfg(test)]
mod test {
    extern crate proptest;

    use super::*;
    use crate::primitives::scalars::scalar::*;
    use crate::primitives::scalars::string_scalar::StringScalar;
    use chrono::Duration;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn is_keyed_before_after(string1 in ".*", string2 in ".*", days in 1i64..1500i64) {
          let now = Utc::now();
          let before = now - Duration::days(days);
          let s1 = Sample{
            value: Scalar::String(StringScalar::of(string1.clone())),
            time: before
          };
          let s2 = Sample{
            value: Scalar::String(StringScalar::of(string2.clone())),
            time: now
          };
          let s3 = Sample{
            value: Scalar::String(StringScalar::of(string2.clone())),
            time: now
          };
          assert_eq!(s1.is_keyed_before(&s2), true);
          assert_eq!(s2.is_keyed_before(&s2), false);
          assert_eq!(s2.is_keyed_before(&s3), false);
          assert_eq!(s1.is_keyed_after(&s2), false);
          assert_eq!(s2.is_keyed_after(&s2), false);
          assert_eq!(s2.is_keyed_after(&s1), true);
        }

        #[test]
        fn is_keyed_at(string1 in ".*", string2 in ".*", days in 1i64..1500i64) {
          let now = Utc::now();
          let later = now + Duration::days(days);
          let s1 = Sample{
            value: Scalar::String(StringScalar::of(string1.clone())),
            time: now
          };
          let s2 = Sample{
            value: Scalar::String(StringScalar::of(string2.clone())),
            time: later
          };
          let s3 = Sample{
            value: Scalar::String(StringScalar::of(string2.clone())),
            time: now
          };
          assert_eq!(s1.is_keyed_at(&s3), true);
          assert_eq!(s1.is_keyed_at(&s2), false);
        }
    }
}
