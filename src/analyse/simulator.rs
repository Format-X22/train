use crate::data::candle::Candle;
use crate::data::database::Database;
use chrono::{DateTime, Local};
use log::info;
use std::cell::{RefCell, RefMut};

pub struct Simulator<'a> {
    database: &'a RefCell<Database>,

    padding_percent: f64,
    stop_percent: f64,
    capital_percent: f64,

    capital: f64,
    fails: i64,
    max_waited: i64,
}

pub struct SimulatorConfig {
    pub padding_percent: f64,
    pub stop_percent: f64,
    pub capital_percent: f64,
}

impl<'a> Simulator<'a> {
    pub fn new(database: &'a RefCell<Database>, config: SimulatorConfig) -> Self {
        Simulator {
            database,
            padding_percent: config.padding_percent,
            stop_percent: config.stop_percent,
            capital_percent: config.capital_percent,
            capital: 100.0,
            fails: 0,
            max_waited: 0,
        }
    }

    pub fn simulate(&self) {
        let candles = self.database.borrow_mut().get_candles(100_000);
        let count = candles.len();

        let first_timestamp = candles.first().unwrap().timestamp;
        let first_date = DateTime::from_timestamp_millis(first_timestamp)
            .unwrap()
            .with_timezone(&Local)
            .format("%Y-%m-%d %H:%M");
        let last_timestamp = candles.last().unwrap().timestamp;
        let last_date = DateTime::from_timestamp_millis(last_timestamp)
            .unwrap()
            .with_timezone(&Local)
            .format("%Y-%m-%d %H:%M");

        info!(
            "Simulate {} hours from {} to {}",
            count, first_date, last_date
        );
        info!(
            "Padding {}%, stop {}%, capital {}%",
            self.padding_percent, self.stop_percent, self.capital_percent
        );

        for candle in candles {
            self.simulate_for(candle);
        }

        self.print_results();
    }

    fn simulate_for(&self, candle: Candle) {
        // TODO -
    }

    fn print_results(&self) {
        // TODO -
    }
}
