use crate::primitives::sample::Sample;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SampleRecord {
    time: DateTime<Utc>,
    value: String,
}

impl SampleRecord {
    fn to_sample(&self) -> Sample {
        unimplemented!()
    }
}
