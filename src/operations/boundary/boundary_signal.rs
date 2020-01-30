use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;
use crate::primitives::scalars::float_scalar::FloatScalar;
use crate::primitives::scalars::scalar::{BaseScalar, Scalar};
use crate::primitives::scalars::string_scalar::StringScalar;
use crate::primitives::scalars::F64::F64;
use crate::primitives::signal::Signal;
use anyhow::Error;
use chrono::Duration;

pub struct BoundarySignal<'a> {
    signal: &'a mut Signal,
}

impl<'a> BoundarySignal<'a> {
    pub fn of(signal: &'a mut Signal) -> Self {
        BoundarySignal { signal }
    }
}

impl<'a> BaseOperation for BoundarySignal<'a> {
    type Primitive = Signal;

    fn apply(&mut self) -> Result<Signal, Error> {
        let samples = self.signal.get_samples();
        let duration;
        if samples.len() == 1 {
            duration = Duration::hours(1);
        } else {
            duration = samples[1].time.signed_duration_since(samples[0].time);
        }

        let value = match samples[0].value {
            Scalar::Float(_) => Scalar::Float(FloatScalar::of(F64::of(std::f64::MIN))),
            Scalar::String(_) => Scalar::String(StringScalar::of("".to_string())),
        };

        let left = Sample::of(value.clone(), samples[0].time - duration);
        let right = Sample::of(value, samples[samples.len() - 1].time + duration);

        samples.insert(0, left);
        samples.push(right);

        Ok(Signal::of(samples.to_vec(), self.signal.is_numeric()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::scalars::float_scalar::FloatScalar;
    use crate::primitives::scalars::scalar::*;
    use crate::primitives::scalars::string_scalar::StringScalar;
    use crate::primitives::scalars::F64::F64;
    use chrono::prelude::*;
    use chrono::Duration;
    use proptest::prelude::*;
    use std::f64::MAX;

    proptest! {
        #[test]
        fn one_sample_string(string1 in ".*") {
          let now = Utc::now();
          let early = now - Duration::hours(1);
          let later = now + Duration::hours(1);
          let s1 = Sample::of(Scalar::String(StringScalar::of(string1.clone())), now);
          let samples1 = vec![s1.clone()];
          let expected_samples = vec![Sample::of(Scalar::String(StringScalar::of("".to_string())), early),
            s1.clone(),
            Sample::of(Scalar::String(StringScalar::of("".to_string())), later)
          ];
          let mut signal1 = Signal::of(samples1.clone(), false);
          let mut result = BoundarySignal::of(&mut signal1).apply().unwrap();
          assert!(expected_samples == *result.get_samples());
        }

        #[test]
        fn multiple_samples_float(float1 in 0f64..MAX/2f64, float2 in 0f64..MAX/2f64) {
            let now = Utc::now();
            let early = now - Duration::days(2);
            let later = now + Duration::days(2);
            let laterer = later + Duration::days(2);
            let s1 = Sample::of(Scalar::Float(FloatScalar::of(F64::of(float1))), now);
            let s2 = Sample::of(Scalar::Float(FloatScalar::of(F64::of(float2))), later);
            let samples1 = vec![s1.clone(), s2.clone()];
            let expected_samples = vec![Sample::of(Scalar::Float(FloatScalar::of(F64::of(std::f64::MIN))), early),
              s1.clone(),
              s2.clone(),
              Sample::of(Scalar::Float(FloatScalar::of(F64::of(std::f64::MIN))), laterer)
            ];
            let mut signal1 = Signal::of(samples1.clone(), false);
            let mut result = BoundarySignal::of(&mut signal1).apply().unwrap();
            for f in &expected_samples {
                println!("{}", f.time);
            }
            for f in result.get_samples() {
                println!("{}", f.time);
            }
            assert!(expected_samples == *result.get_samples());
        }
    }
}
