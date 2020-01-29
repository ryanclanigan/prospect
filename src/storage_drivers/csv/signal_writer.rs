use super::sample_record::SampleRecord;
use crate::primitives::sample::Sample;
use crate::primitives::signal::Signal;
use anyhow::Result;
use csv::Writer;
use std::path::Path;

pub struct SignalWriter;

impl SignalWriter {
    const SAMPLE_WRITE_MESSAGE: &'static str =
        "Samples are out of order or have duplicate keys. Please make sure your data is accurate";

    pub fn write_signal(&self, signal: &mut Signal, file: &Path) -> Result<()> {
        let mut writer = Writer::from_path(file)?;
        let mut iter = signal.get_samples().iter();
        let first_sample = match iter.next() {
            Some(s) => s,
            None => return Err(anyhow!("Signal does not have any samples")),
        };
        writer.serialize(SampleRecord::from_sample(first_sample))?;
        let mut last_sample: Sample = first_sample.clone();
        for sample in iter {
            if !last_sample.is_keyed_before(sample) {
                return Err(anyhow!(Self::SAMPLE_WRITE_MESSAGE));
            }
            let record = SampleRecord::from_sample(sample);
            writer.serialize(record)?;
            last_sample = sample.clone();
        }

        writer.flush()?;
        Ok(())
    }
}
