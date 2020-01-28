use crate::primitives::item::Item;
use crate::primitives::signal::Signal;
use crate::storage_drivers::csv::signal_reader::SignalReader;
use crate::storage_drivers::csv::signal_writer::SignalWriter;
use anyhow::Error;
use std::fs;
use std::path::Path;

pub struct SignalSerializer;

impl SignalSerializer {
    const dataFolder: &'static str = "./data";

    pub fn new() -> Result<Self, Error> {
        let data_dir = Path::new(Self::dataFolder);
        if data_dir.exists() == false {
            fs::create_dir(data_dir)?;
        }
        Ok(SignalSerializer)
    }

    pub fn write(&self, signal: &mut Signal) -> Result<(), Error> {
        let writer = SignalWriter;
        Ok(writer.write_signal(
            signal,
            &Path::new(Self::dataFolder).join(format!("{}{}", signal.get_id().to_string(), ".csv")),
        )?)
    }

    pub fn read(&self, id: String) -> Result<Signal, Error> {
        let reader = SignalReader;
        Ok(reader.read_signal(
            &Path::new(Self::dataFolder).join(format!("{}{}", id, ".csv")),
            0,
            1,
        )?)
    }

    pub fn get_all_signal_ids(&self) -> Result<Signal, Error> {}
}
