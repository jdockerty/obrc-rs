/// One billion rows challenge.
///
/// Read the contents of [`OBRC_PATH`] and output
/// the mean/min/max of the measurements per station,
/// sorted in alphabetical order.
use std::{fs::File, io::BufReader, time::Instant};

use crate::obrc::read_stations;
use ::obrc::OBRC_PATH;

pub mod obrc;

fn main() {
    let mut reader = BufReader::new(File::open(OBRC_PATH).unwrap());
    let start = Instant::now();

    let results = read_stations(&mut reader);
    for (station, value) in results {
        println!("{station}={value}");
    }

    println!("{}ms", start.elapsed().as_millis());
}
