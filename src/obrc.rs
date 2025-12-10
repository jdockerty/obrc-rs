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

pub fn read_stations<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> BTreeMap<String, ChallengeValue> {
    // Using a [`BTreeMap`] means that the station locations
    // are automatically held in alphabetically sorted order.
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

        let results = read_stations(&mut reader);
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
