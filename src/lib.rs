pub mod obrc;

pub const OBRC_PATH: &str = "testdata/weather_stations.csv";
pub use obrc::read_stations;
