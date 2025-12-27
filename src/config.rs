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
    pub candle_size: String,
    pub risk_deduction: f64,
    pub padding_percent_sell: f64,
    pub padding_percent_buy: f64,
    pub capital_percent: f64,
    pub order_decimals: usize,
    pub price_decimals: usize,
    pub is_simulation: bool,
    pub simulate_from: String,
}
