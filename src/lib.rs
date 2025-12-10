pub mod obrc;

// Default path, specific to this repository.
pub const DEFAULT_OBRC_PATH: &str = "1b-measurements.txt";
pub use obrc::read_stations;
