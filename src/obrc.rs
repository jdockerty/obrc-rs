use std::{
    collections::BTreeMap,
    fmt::Display,
    io::{BufRead, BufReader, Read, Seek},
};

#[derive(Debug, PartialEq)]
pub struct ChallengeValue {
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

fn chunk_to_stations(buf: String) -> BTreeMap<String, ChallengeValue> {
    let mut results = BTreeMap::new();
    for entry in buf.split('\n') {
        let entry = entry.split(";").collect::<Vec<_>>();

        if entry.len() == 1 {
            break;
        }

        let station = entry[0].to_string();
        let measurement = entry[1]
            .trim()
            .parse::<f64>()
            .expect("valid measurements given");

        results
            .entry(station)
            .and_modify(|s: &mut ChallengeValue| {
                s.total += measurement;
                s.count += 1;

                s.min = s.min.min(measurement);
                s.max = s.max.max(measurement);
                s.mean = s.total / (s.count as f64);
            })
            .or_insert_with(|| ChallengeValue::new(measurement));
    }
    results
}

pub fn read_stations<R: Read + Seek>(
    reader: &mut BufReader<R>,
    size_hint: u64,
) -> BTreeMap<String, ChallengeValue> {
    let mut workers = Vec::new();

    let max_parallelism = std::thread::available_parallelism().unwrap().get() as u64;
    let max_chunk_size = size_hint / max_parallelism;
    let mut entries = 0;

    let mut buf = String::new();
    while let Ok(v) = reader.read_line(&mut buf) {
        if v == 0 {
            break;
        }

        if entries == max_chunk_size {
            let buf2 = buf.clone();
            let handle = std::thread::spawn(move || chunk_to_stations(buf2));
            workers.push(handle);
            buf.clear();
            entries = 0;
        }
        entries += 1;
    }

    // If we didn't reach the max chunk size, but the buf is still not cleared,
    // then should dispatch a worker to handle it.
    if !buf.is_empty() {
        let buf2 = buf.clone();
        let handle = std::thread::spawn(move || chunk_to_stations(buf2));
        workers.push(handle);
        buf.clear();
    }

    let mut results = BTreeMap::new();
    for worker in workers {
        results.extend(worker.join().unwrap());
    }

    results
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader};

    use crate::{obrc::ChallengeValue, read_stations};

    const SMALL_INPUT_PATH: &str = "testdata/small.txt";

    #[test]
    fn correct_output() {
        let f = File::open(SMALL_INPUT_PATH).unwrap();
        let mut reader = BufReader::new(f);

        let results = read_stations(&mut reader, 3);
        assert_eq!(results.len(), 2);
        assert_eq!(
            *results.get("a").unwrap(),
            ChallengeValue {
                min: 1.0,
                mean: 1.0,
                max: 1.0,
                total: 1.0,
                count: 1
            }
        );
        assert_eq!(
            *results.get("b").unwrap(),
            ChallengeValue {
                min: 1.0,
                mean: 1.5,
                max: 2.0,
                total: 3.0,
                count: 2
            }
        );
    }
}
