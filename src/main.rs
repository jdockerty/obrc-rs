/// One billion rows challenge.
///
/// Read the contents of [`OBRC_PATH`] and output
/// the mean/min/max of the measurements per station,
/// sorted in alphabetical order.
use std::{fmt::Write, fs::File, io::BufReader};

use crate::obrc::read_stations;
use ::obrc::DEFAULT_OBRC_PATH;

pub mod obrc;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let measurement_path = args
        .get(1)
        .cloned()
        .unwrap_or(DEFAULT_OBRC_PATH.to_string());

    let mut reader = BufReader::new(File::open(measurement_path).unwrap());

    let results = read_stations(&mut reader);
    let mut buf = String::with_capacity(1024 * 1024 * 1024);
    for (station, value) in results {
        buf.write_fmt(format_args!("{station} {value}")).unwrap();
    }

    println!("{buf}");
}
