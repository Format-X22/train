use log::LevelFilter;
use serde::Deserialize;

pub const ENV_PREFIX: &str = "T_";

#[derive(Deserialize)]
#[serde(remote = "LevelFilter")]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(with = "LogLevel")]
    pub log_level: LevelFilter,
    pub public_key: String,
    pub private_key: String,
    pub is_simulator: bool,
    pub padding_percent: f64,
    pub stop_percent: f64,
    pub capital_percent: f64,
}
