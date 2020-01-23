use super::add_scalar::*;
use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;

pub struct AddSample<'a> {
    args: Box<[&'a Sample]>,
}

impl<'a> AddSample<'a> {
    pub fn of(sample1: &'a Sample, sample2: &'a Sample) -> Self {
        AddSample {
            args: Box::new([sample1, sample2]),
        }
    }
}

impl<'a> BaseOperation<Sample> for AddSample<'a> {
    fn get_op(&self) -> Box<dyn Fn(&Box<[&Sample]>) -> Result<Sample, &'static str>> {
        Box::new(|args: &Box<[&Sample]>| -> Result<Sample, &'static str> {
            let sample1 = args[0];
            let sample2 = args[1];
            if sample1.time.eq(&sample2.time) == true {
                let op = AddScalar::of(&sample1.value, &sample2.value);
                let result = op.apply();
                match result {
                    Ok(s) => {
                        return Ok(Sample {
                            value: s,
                            time: sample1.time,
                        })
                    }
                    Err(e) => return Err(e),
                }
            }
            Err("Sample times of two samples did not match")
        })
    }

    fn get_args(&self) -> &Box<[&Sample]> {
        &self.args
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
          let s1 = Sample{
            value: Scalar::String(StringScalar::of(string1.clone())),
            time: now
          };
          let s2 = Sample{
            value: Scalar::String(StringScalar::of(string2.clone())),
            time: now
          };
          let op = AddSample::of( &s1, &s2);
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
          let op = AddSample::of(&s1, &s2);
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
