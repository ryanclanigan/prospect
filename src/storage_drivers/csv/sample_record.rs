use crate::primitives::sample::Sample;
use crate::primitives::scalars::float_scalar::FloatScalar;
use crate::primitives::scalars::scalar::*;
use crate::primitives::scalars::string_scalar::StringScalar;
use crate::primitives::scalars::F64::F64;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Serialize)]
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
