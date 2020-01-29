use crate::primitives::item::Item;
use crate::primitives::signal::Signal;
use crate::storage_drivers::csv::signal_reader::SignalReader;
use crate::storage_drivers::csv::signal_writer::SignalWriter;
use anyhow::Error;
use std::fs;
use std::path::Path;

pub struct SignalSerializer;

impl SignalSerializer {
    const DATA_FOLDER: &'static str = "./data";

    pub fn new() -> Self {
        SignalSerializer
    }

    pub fn write(&self, signal: &mut Signal) -> Result<(), Error> {
        let writer = SignalWriter;
        Ok(writer.write_signal(
            signal,
            &Path::new(Self::DATA_FOLDER).join(format!(
                "{}{}",
                signal.get_id().to_string(),
                ".csv"
            )),
        )?)
    }

    pub fn read(&self, id: String) -> Result<Signal, Error> {
        let reader = SignalReader;
        Ok(reader.read_signal(
            &Path::new(Self::DATA_FOLDER).join(format!("{}{}", id, ".csv")),
            0,
            1,
        )?)
    }

    pub fn get_all_signal_ids(&self) -> Result<Vec<String>, Error> {
        let paths = fs::read_dir(Self::DATA_FOLDER)?;

        let mut files: Vec<String> = Vec::new();
        for path in paths {
            if let Some(os_path) = path?.path().file_stem() {
                if let Some(real_file_name) = os_path.to_str() {
                    files.push(real_file_name.to_string());
                }
            };
        }
        Ok(files)
    }

    pub fn init_once() -> Result<(), Error> {
        let data_dir = Path::new(Self::DATA_FOLDER);
        if data_dir.exists() == false {
            fs::create_dir(data_dir)?;
        }
        Ok(())
    }
}
