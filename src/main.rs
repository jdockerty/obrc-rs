/// One billion rows challenge.
///
/// Read the contents of [`OBRC_PATH`] and output
/// the mean/min/max of the measurements per station,
/// sorted in alphabetical order.
use std::{fmt::Write, fs::File, io::BufReader};

use crate::obrc::read_stations;
use ::obrc::DEFAULT_OBRC_PATH;

pub mod obrc;

struct SizeHint(String);

impl SizeHint {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }

    pub fn get(&self) -> u64 {
        match self.0.to_lowercase().as_str() {
            "1k" => 1_000,
            "10k" => 10_000,
            "100k" => 100_000,
            "1m" => 1_000_000,
            "10m" => 10_000_000,
            "100m" => 10_000_000,
            "1b" => 1_000_000_000,
            _ => 10_000,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let measurement_path = args
        .get(1)
        .cloned()
        .unwrap_or(DEFAULT_OBRC_PATH.to_string());

    let size_hint = SizeHint::new(args.get(2).cloned().unwrap_or("10k".to_string()));

    let file = File::open(measurement_path).unwrap();
    let mut reader = BufReader::new(file);

    let results = read_stations(&mut reader, size_hint.get());
    let mut buf = String::with_capacity(1024 * 1024 * 1024);
    for (station, value) in results {
        buf.write_fmt(format_args!("{station} {value}")).unwrap();
    }

    println!("{buf}");
}
