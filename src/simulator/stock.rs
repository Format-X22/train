use std::collections::HashMap;
use snafu::{ResultExt, Whatever};
use crate::candle::Candle;
use crate::dto::CandlesData;
use crate::stock::Stock;

impl Stock {
    pub fn get_candles_from(&self, from: i64, ticker: &str) -> Result<Vec<Candle>, Whatever> {
        let point = "market/kline";
        let from = from.to_string();
        let params = HashMap::from([
            ("category", "linear"),
            ("symbol", ticker),
            ("interval", "60"),
            ("start", &from),
            ("limit", "1000"),
        ]);

        let response = self.call_with_get(point, params)?;
        let parsed = response
            .json::<CandlesData>()
            .whatever_context("On parse candles data")?;
        let raw = self.check_and_extract_data(parsed)?;
        let mut data = vec![];

        for raw in raw.list {
            data.push(Candle::from_raw(raw)?)
        }
        data.reverse();

        Ok(data)
    }
}