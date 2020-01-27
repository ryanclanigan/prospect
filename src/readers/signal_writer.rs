use super::sample_record::SampleRecord;
use crate::primitives::sample::Sample;
use crate::primitives::scalars::scalar::Scalar;
use crate::primitives::signal::Signal;
use anyhow::Result;
use csv::StringRecord;
use csv::Writer;
use serde::Serialize;
use std::path::Path;

pub struct SignalWriter;

impl SignalWriter {
    pub fn write_signal(&self, signal: &mut Signal, file: &Path) -> Result<()> {
        let mut writer = Writer::from_path(file)?;
        for sample in signal.get_samples() {
            let record = SampleRecord::from_sample(sample);
            writer.serialize(record);
        }

        writer.flush()?;
        Ok(())
    }
}
