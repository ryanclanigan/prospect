#[macro_use]
extern crate anyhow;
extern crate chrono;
extern crate csv;
extern crate serde;

mod operations;
mod primitives;
mod readers;

use chrono::prelude::*;
use chrono::DateTime;
use primitives::sample::Sample;
use readers::signal_reader::SignalReader;
use readers::signal_writer::SignalWriter;

fn main() {
    let r = SignalReader;
    let mut s = r.read_signal(std::path::Path::new("g.csv"), 0, 1).unwrap();
    let w = SignalWriter;
    w.write_signal(&mut s, std::path::Path::new("h.csv"))
        .unwrap();
}
