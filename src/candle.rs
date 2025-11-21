use serde::Deserialize;
use snafu::{ResultExt, Whatever};

#[derive(Copy, Clone, Debug)]
pub struct Candle {
    pub timestamp: i64,
    pub open: f64,
}

impl Candle {
    pub fn from_raw(raw: RawCandle) -> Result<Candle, Whatever> {
        Ok(Candle {
            timestamp: raw.0.parse().whatever_context("On parse timestamp")?,
            open: raw.1.parse().whatever_context("On parse open")?,
        })
    }
}

#[derive(Deserialize)]
pub struct RawCandle(
    pub String,                     // Date
    pub String,                     // Open
    #[allow(dead_code)] pub String, // High
    #[allow(dead_code)] pub String, // Low
    #[allow(dead_code)] pub String, // Close
    #[allow(dead_code)] String,     // Volume
    #[allow(dead_code)] String,     // Turnover
);
