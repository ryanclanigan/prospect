use crate::primitives::sample::Sample;
use crate::primitives::scalars::float_scalar::FloatScalar;
use crate::primitives::scalars::scalar::*;
use crate::primitives::scalars::string_scalar::StringScalar;
use crate::primitives::scalars::F64::F64;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SampleRecord {
    time: String,
    value: String,
}

impl SampleRecord {
    pub fn to_sample(&self) -> Result<Sample> {
        let maybe_float = F64::try_from(&self.value);
        let scalar: Scalar = match maybe_float {
            Ok(f) => Scalar::Float(FloatScalar::of(f)),
            Err(_) => Scalar::String(StringScalar::of(self.value.clone())),
        };
        let time: DateTime<Utc> = DateTime::parse_from_rfc3339(&self.time)?.with_timezone(&Utc);
        Ok(Sample::of(scalar, time))
    }

    pub fn from_sample(sample: &Sample) -> SampleRecord {
        SampleRecord {
            time: sample.time.to_rfc3339(),
            value: match &sample.value {
                Scalar::Float(f) => f.to_value().to_value().to_string(),
                Scalar::String(s) => s.to_value(),
            },
        }
    }

    pub fn from_values(time: String, value: String) -> Self {
        SampleRecord { time, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn get_string_sample_and_now(s: String) -> (Sample, DateTime<Utc>) {
        let now = Utc::now();
        (Sample::of(Scalar::String(StringScalar::of(s)), now), now)
    }

    fn get_numeric_sample_and_now(f: f64) -> (Sample, DateTime<Utc>) {
        let now = Utc::now();
        (
            Sample::of(Scalar::Float(FloatScalar::of(F64::of(f))), now),
            now,
        )
    }

    proptest! {
        #[test]
        fn test_from_value(string1 in ".*", string2 in ".*") {
            let record = SampleRecord::from_values(string1.clone(), string2.clone());
            assert_eq!(string1, record.time);
            assert_eq!(string2, record.value);
        }

        #[test]
        fn test_from_string_sample(string1 in ".*") {
            let (sample, now) = get_string_sample_and_now(string1.clone());
            let record = SampleRecord::from_sample(&sample);
            assert_eq!(record.time, now.to_rfc3339());
            assert_eq!(record.value, string1);
        }

        #[test]
        fn test_from_numeric_sample(f in prop::num::f64::ANY) {
            let (sample, now) = get_numeric_sample_and_now(f);
            let record = SampleRecord::from_sample(&sample);
            assert_eq!(record.time, now.to_rfc3339());
            assert_eq!(F64::try_from(&record.value).unwrap(), F64::of(f));
        }

        #[test]
        fn test_to_string_sample(string1 in ".*") {
            let (sample, now) = get_string_sample_and_now(string1.clone());
            let record = SampleRecord::from_values(now.to_rfc3339(), string1);

            assert_eq!(sample, record.to_sample().unwrap());
        }

        #[test]
        fn test_to_numeric_sample(f in prop::num::f64::ANY) {
            let (sample, now) = get_numeric_sample_and_now(f);
            let record = SampleRecord::from_values(now.to_rfc3339(), f.to_string());

            assert_eq!(sample, record.to_sample().unwrap());
        }
    }
}
