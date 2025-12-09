use std::{fs::File, io::BufReader};

use criterion::{Criterion, criterion_group, criterion_main};
use obrc::{OBRC_PATH, read_stations};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("one billion rows challenge", |b| {
        let mut reader = BufReader::new(File::open(OBRC_PATH).unwrap());
        b.iter(|| {
            read_stations(&mut reader);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
