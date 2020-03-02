use super::sample_record::SampleRecord;
use crate::primitives::sample::Sample;
use crate::primitives::scalars::scalar::Scalar;
use crate::primitives::signal::Signal;
use anyhow::Result;
use csv::Reader;
use csv::StringRecord;
use std::path::Path;

pub struct SignalReader;

impl SignalReader {
    pub fn read_signal(
        &self,
        file: &Path,
        time_column: usize,
        value_column: usize,
    ) -> Result<Signal> {
        let mut reader = Reader::from_path(file)?;
        let file_as_str = match file.to_str() {
            Some(f) => f,
            None => return Err(anyhow!("CSV file not found")),
        };
        let mut samples: Vec<Sample> = Vec::new();

        let mut iter = reader.records();
        let first_sample = match iter.next() {
            Some(s) => {
                let record = s?;
                let time = self.get_value_from_column(&record, time_column, "Time", file_as_str)?;
                let value =
                    self.get_value_from_column(&record, value_column, "Value", file_as_str)?;
                SampleRecord::from_values(time, value).to_sample()?
            }
            None => return Err(anyhow!("Tried to read from a csv file with no data")),
        };
        let numeric = match first_sample.value {
            Scalar::Float(_) => true,
            Scalar::String(_) => false,
        };

        samples.push(first_sample);

        for result in iter {
            let record = result?;
            let time = self.get_value_from_column(&record, time_column, "Time", file_as_str)?;
            let value = self.get_value_from_column(&record, value_column, "Value", file_as_str)?;
            samples.push(
                self.check_record_to_sample_numeric(
                    SampleRecord::from_values(time, value),
                    numeric,
                )?,
            );
        }

        Ok(Signal::of(samples, numeric))
    }

    fn check_record_to_sample_numeric(
        &self,
        record: SampleRecord,
        numeric: bool,
    ) -> Result<Sample> {
        let sample = record.to_sample()?;
        if sample.is_numeric() != numeric {
            return Err(anyhow!("Not all samples in file are of same type"));
        }
        Ok(sample)
    }

    fn get_value_from_column<'a>(
        &self,
        record: &StringRecord,
        column: usize,
        error_prefix: &'static str,
        file_as_str: &'a str,
    ) -> Result<String> {
        match record.get(column) {
            Some(t) => Ok(t.to_string()),
            None => Err(anyhow!(
                "{}. column {} in csv file {} not found",
                error_prefix,
                column,
                file_as_str
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::scalars::float_scalar::FloatScalar;
    use crate::primitives::scalars::scalar::BaseScalar;
    use crate::primitives::scalars::string_scalar::StringScalar;
    use crate::primitives::scalars::F64::F64;
    use chrono::prelude::*;

    #[test]
    fn test_check_record_to_sample_numeric() {
        let numeric_sample = Sample::of(Scalar::Float(FloatScalar::of(F64::of(1f64))), Utc::now());
        let string_sample = Sample::of(
            Scalar::String(StringScalar::of("Fish".to_string())),
            Utc::now(),
        );

        let numeric_sample_record = SampleRecord::from_sample(&numeric_sample);
        let string_sample_record = SampleRecord::from_sample(&string_sample);

        let reader = SignalReader;

        assert_eq!(
            numeric_sample.clone(),
            reader
                .check_record_to_sample_numeric(numeric_sample_record.clone(), true)
                .unwrap()
        );
        match reader.check_record_to_sample_numeric(numeric_sample_record.clone(), false) {
            Err(e) => assert_eq!("Not all samples in file are of same type", e.to_string()),
            _ => unreachable!(),
        };
        assert_eq!(
            string_sample.clone(),
            reader
                .check_record_to_sample_numeric(string_sample_record.clone(), false)
                .unwrap()
        );
        match reader.check_record_to_sample_numeric(string_sample_record, true) {
            Err(e) => assert_eq!("Not all samples in file are of same type", e.to_string()),
            _ => unreachable!(),
        };
    }
}
