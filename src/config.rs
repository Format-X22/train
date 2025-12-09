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
    pub ticker: String,
    pub padding_percent: f64,
    pub order_sell_size: f64,
    pub order_buy_size: f64,
    pub order_decimals: usize,
    pub is_simulation: bool,
}
