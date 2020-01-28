use super::add_scalar::*;
use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;

pub struct AddSample<'a> {
    sample1: &'a Sample,
    sample2: &'a Sample,
}

impl<'a> AddSample<'a> {
    pub fn of(sample1: &'a Sample, sample2: &'a Sample) -> Self {
        AddSample { sample1, sample2 }
    }
}

impl<'a> BaseOperation<Sample> for AddSample<'a> {
    fn apply(&mut self) -> Result<Sample, &'static str> {
        if self.sample1.time.eq(&self.sample2.time) {
            let mut op = AddScalar::of(&self.sample1.value, &self.sample2.value);
            let result = op.apply();
            match result {
                Ok(s) => return Ok(Sample::of(s, self.sample1.time)),
                Err(e) => return Err(e),
            }
        }
        Err("Sample times of two samples did not match")
    }
}

#[cfg(test)]
mod test {
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
          let s1 = Sample{
            value: Scalar::String(StringScalar::of(string1.clone())),
            time: now
          };
          let s2 = Sample{
            value: Scalar::String(StringScalar::of(string2.clone())),
            time: now
          };
          let mut op = AddSample::of( &s1, &s2);
          assert_eq!(
            format!("{}{}",string1,string2),
            match op.apply().unwrap().value {
              Scalar::String(s) => s.to_value(),
              _ => unreachable!(),
            }
          );
        }

        #[test]
        fn add_times_do_not_match(string1 in ".*", string2 in ".*", days in 1i64..1500i64) {
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
          let mut op = AddSample::of(&s1, &s2);
          assert_eq!(
            format!("Sample times of two samples did not match"),
            match op.apply() {
              Err(e) => e,
              _ => unreachable!(),
            }
          );
        }
    }
}
