use crate::primitives::item::Item;
use crate::primitives::signal::Signal;
use crate::storage_drivers::csv::signal_reader::SignalReader;
use crate::storage_drivers::csv::signal_writer::SignalWriter;
use actix_multipart::Field;
use anyhow::Error;
use futures::stream::StreamExt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct SignalSerializer;

impl SignalSerializer {
    const DATA_FOLDER: &'static str = "./data";
    const TEMP_FOLDER: &'static str = "./temp";

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

    pub fn read_temp(&self, name: String) -> Result<Signal, Error> {
        let reader = SignalReader;
        Ok(reader.read_signal(
            &Path::new(Self::TEMP_FOLDER).join(format!("{}", name)),
            0,
            1,
        )?)
    }

    // TODO Make unwraps not be here
    pub async fn write_temp_from_bytes(&self, filename: &str, mut field: Field) -> String {
        let filepath = Path::new(Self::TEMP_FOLDER).join(filename);
        let mut f = fs::File::create(&filepath).unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }
        filepath.to_str().unwrap().to_string()
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
        if !data_dir.exists() {
            fs::create_dir(data_dir)?;
        }
        let temp_dir = Path::new(Self::TEMP_FOLDER);
        if !temp_dir.exists() {
            fs::create_dir(temp_dir)?;
        }
        Ok(())
    }

    pub fn get_raw_file(&self, id: String) -> Result<PathBuf, Error> {
        let file = Path::new(Self::DATA_FOLDER).join(format!("{}{}", id, ".csv"));
        match file.exists() {
            true => Ok(file),
            false => Err(anyhow!("No file with that ID")),
        }
    }
}
