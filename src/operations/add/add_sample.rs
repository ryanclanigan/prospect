use super::add_scalar::AddScalar;
use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;

pub struct AddSample {
  value: Sample,
}

impl BaseOperation<Sample> for AddSample {
  fn apply(&self, sample: &Sample) -> Result<Sample, &'static str> {
    if self.value.time.eq(&sample.time) == true {
      let op = AddScalar {
        value: &self.value.value,
      };
      let result = op.apply(&sample.value);
      match result {
        Ok(s) => {
          return Ok(Sample {
            value: s,
            time: self.value.time,
          })
        }
        Err(e) => return Err(e),
      }
    }
    Err("Sample times of two samples did not match")
  }
}

#[cfg(test)]
mod test {

  extern crate chrono;
  extern crate proptest;

  use super::*;
  use crate::primitives::scalars::scalar::*;
  use crate::primitives::scalars::string_scalar::StringScalar;
  use chrono::prelude::*;
  use chrono::Duration;
  use proptest::prelude::*;

  proptest! {
      #[test]
      fn add_happy_path(string1 in ".*", string2 in ".*") {
        let now = Utc::now();
        let s1 = AddSample {
          value: Sample{
            value: Scalar::String(StringScalar::of(string1.clone())),
            time: now
          },
        };
        let s2 = Sample{
          value: Scalar::String(StringScalar::of(string2.clone())),
          time: now
        };
        assert_eq!(
          format!("{}{}",string1,string2),
          match s1.apply(&s2).unwrap().value {
            Scalar::String(s) => s.to_value(),
            _ => unreachable!(),
          }
        );
      }

      #[test]
      fn add_times_do_not_match(string1 in ".*", string2 in ".*", days in 1i64..1500i64) {
        let now = Utc::now();
        let later = now + Duration::days(days);
        let s1 = AddSample {
          value: Sample{
            value: Scalar::String(StringScalar::of(string1.clone())),
            time: now
          },
        };
        let s2 = Sample{
          value: Scalar::String(StringScalar::of(string2.clone())),
          time: later
        };
        assert_eq!(
          format!("Sample times of two samples did not match"),
          match s1.apply(&s2) {
            Err(e) => e,
            _ => unreachable!(),
          }
        );
      }
  }
}
