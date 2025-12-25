use serde::Deserialize;
use snafu::{ResultExt, Whatever};

#[derive(Copy, Clone, Debug)]
pub struct Candle {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl Candle {
    pub fn from_raw(raw: RawCandle) -> Result<Candle, Whatever> {
        Ok(Candle {
            timestamp: raw.0.parse().whatever_context("On parse timestamp")?,
            open: raw.1.parse().whatever_context("On parse open")?,
            high: raw.2.parse().whatever_context("On parse high")?,
            low: raw.3.parse().whatever_context("On parse low")?,
            close: raw.4.parse().whatever_context("On parse close")?,
        })
    }
}

#[derive(Deserialize)]
pub struct RawCandle(
    pub String,                     // Date
    pub String,                     // Open
    pub String, // High
    pub String, // Low
    pub String, // Close
    #[allow(dead_code)] String,     // Volume
    #[allow(dead_code)] String,     // Turnover
);
