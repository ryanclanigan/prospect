use crate::operations::operation::BaseOperation;
use crate::primitives::sample::Sample;
use crate::primitives::scalars::float_scalar::FloatScalar;
use crate::primitives::scalars::scalar::{BaseScalar, Scalar};
use crate::primitives::scalars::F64::F64;
use crate::primitives::signal::Signal;
use anyhow::Error;
use chrono::Utc;

pub struct ExtremesSignal<'a> {
    signal: &'a mut Signal,
}

/// When applied, returns a signal with the min and max as the first and second samples of the resulting signal
/// The resulting time is nonsensical and shouldn't be used
impl<'a> ExtremesSignal<'a> {
    pub fn of(signal: &'a mut Signal) -> Self {
        ExtremesSignal { signal }
    }
}

impl<'a> BaseOperation for ExtremesSignal<'a> {
    type Primitive = Signal;

    fn apply(&mut self) -> Result<Signal, Error> {
        if !self.signal.is_numeric() {
            return Err(anyhow!("Non-numeric signals do not have extremes"));
        }

        let samples = self.signal.get_samples();
        let mut min = F64::of(std::f64::MAX);
        let mut max = F64::of(std::f64::MIN);
        let mut result: Vec<Sample> = vec![
            Sample::of(Scalar::Float(FloatScalar::of(min)), Utc::now()),
            Sample::of(Scalar::Float(FloatScalar::of(max)), Utc::now()),
        ];

        for sample in samples {
            match sample.value {
                Scalar::Float(f) => {
                    let real_f = f.to_value();
                    if real_f < min {
                        min = real_f;
                        result[0] = sample.clone();
                    }
                    if real_f > max {
                        max = real_f;
                        result[1] = sample.clone();
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(Signal::of(result, true))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::scalars::float_scalar::FloatScalar;
    use crate::primitives::scalars::scalar::*;
    use crate::primitives::scalars::string_scalar::StringScalar;
    use crate::primitives::scalars::F64::F64;
    use proptest::prelude::*;
    use std::f64::MAX;

    proptest! {
        #[test]
        fn string(string1 in ".*") {
          let now = Utc::now();
          let s1 = Sample::of(Scalar::String(StringScalar::of(string1.clone())), now);
          let samples1 = vec![s1.clone()];
          let mut signal1 = Signal::of(samples1.clone(), false);
          let result = ExtremesSignal::of(&mut signal1).apply();
          match result {
              Err(e) => assert_eq!(e.to_string(), "Non-numeric signals do not have extremes"),
              _ => unreachable!(),
          }
        }

        #[test]
        fn float(float1 in 0f64..MAX/3f64, float2 in MAX/3f64..MAX/2f64) {
            let now = Utc::now();
            let s1 = Sample::of(Scalar::Float(FloatScalar::of(F64::of(float1))), now);
            let max = Sample::of(Scalar::Float(FloatScalar::of(F64::of(float2))), now);
            let min = Sample::of(Scalar::Float(FloatScalar::of(F64::of(-1f64))), now);
            let samples1 = vec![max.clone(), s1.clone(), min.clone()];
            let expected_samples = vec![min.clone(), max.clone()];
            let mut signal1 = Signal::of(samples1.clone(), true);
            let mut result = ExtremesSignal::of(&mut signal1).apply().unwrap();
            assert!(expected_samples == *result.get_samples());
        }
    }
}
