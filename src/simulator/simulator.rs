use crate::candle::Candle;
use crate::simulator::database;
use crate::stock::Stock;
use chrono::{DateTime, NaiveDate, Utc};
use log::info;

pub struct Simulator {
    stock: Stock,
    ticker: String,
    padding_percent: f64,
    capital_percent: f64,
    candle_size: i64,
    risk_deduction: f64,
    order_decimals: usize,
    price_decimals: usize,
    simulate_from_ms: i64,
}

impl Simulator {
    pub fn new(
        stock: Stock,
        ticker: String,
        padding_percent: f64,
        capital_percent: f64,
        candle_size: i64,
        risk_deduction: f64,
        order_decimals: usize,
        price_decimals: usize,
        simulate_from: String,
    ) -> Self {
        Self {
            stock,
            ticker,
            padding_percent,
            capital_percent,
            candle_size,
            risk_deduction,
            order_decimals,
            price_decimals,
            simulate_from_ms: NaiveDate::parse_from_str(&simulate_from, "%Y-%m-%d")
                .expect("Invalid 'simulate from' date")
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_millis(),
        }
    }

    pub fn run(&mut self) {
        let candles = self.get_candles();

        info!("{} candles in simulation", candles.len());

        for candle in candles {
            // TODO Check collisions
            // TODO Check inline collisions
            // TODO Place new orders
            // TODO Calc orders max count, leverage, liquidation price
        }

        // TODO Print results
    }

    fn get_candles(&self) -> Vec<Candle> {
        let database = database::Database::new(&self.ticker);

        info!("Syncing candles...");
        Candle::sync(&database, &self.stock, &self.ticker, self.candle_size);
        info!("Sync completed");

        Candle::get_candles(&database, self.simulate_from_ms)
    }
}
