use std::{
    collections::BTreeMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom::Start},
    time::Instant,
};

/// One billion rows challenge.
///
/// Read the contents of [`OBRC_PATH`] and output
/// the mean/min/max of the measurements per station,
/// sorted in alphabetical order.

const OBRC_PATH: &str = "testdata/weather_stations.csv";

const HEADER_BYTES: u64 = 153;

struct ChallengeValue {
    min: f64,
    mean: f64,
    max: f64,

    total: f64,
    count: u64,
}

impl Display for ChallengeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.min, self.mean, self.max)
    }
}

impl ChallengeValue {
    fn new(init: f64) -> Self {
        Self {
            min: init,
            mean: init,
            max: init,
            total: init,
            count: 1,
        }
    }
}

fn main() {
    let start = Instant::now();
    let mut reader = BufReader::new(File::open(OBRC_PATH).unwrap());

    // There are 153 bytes of header, which was read via 'wc -c'
    // that we know we can skip over at the beginning.
    reader.seek(Start(HEADER_BYTES)).unwrap();

    let mut results: BTreeMap<String, ChallengeValue> = BTreeMap::new();

    let mut buf = String::new();
    while let Ok(v) = reader.read_line(&mut buf) {
        if v == 0 {
            break;
        }

        let entry = buf.split(";").collect::<Vec<_>>();

        let station = entry[0].to_string();
        let measurement = entry[1]
            .trim()
            .parse::<f64>()
            .expect("valid measurements given");

        results
            .entry(station)
            .and_modify(|s| {
                s.total += measurement;
                s.count += 1;

                s.min = s.min.min(measurement);
                s.max = s.max.max(measurement);
                s.mean = s.total / (s.count as f64);
            })
            .or_insert_with(|| ChallengeValue::new(measurement));

        buf.clear();
    }

    for (station, value) in results {
        println!("{station}={value}");
    }

    println!("{}ms", start.elapsed().as_millis());
}
