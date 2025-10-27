use crate::data::database::Database;
use crate::trade::stock::Stock;
use chrono::DateTime;
use log::{error, info};
use std::cell::RefCell;
use std::thread::sleep;
use std::time::Duration;

pub struct CandleSync {
    database: RefCell<Database>,
    stock: RefCell<Stock>,
}

const SYNC_FROM: &str = "2021-06-01T00:00:00-00:00";

impl CandleSync {
    pub fn new(database: RefCell<Database>, stock: RefCell<Stock>) -> Self {
        Self { database, stock }
    }

    pub fn sync(&self) {
        let database = self.database.borrow_mut();
        let stock = self.stock.borrow_mut();
        let mut last_timestamp = database.get_last_candle_timestamp();
        let mut is_full_sync = false;

        if last_timestamp == 0 {
            info!("Full sync required, start...");
            let sync_from = DateTime::parse_from_rfc3339(SYNC_FROM).unwrap();
            last_timestamp = sync_from.timestamp_millis();
            is_full_sync = true;
        }

        loop {
            let candles = match stock.get_candles(last_timestamp - 1) {
                Ok(candles) => candles,
                Err(err) => {
                    error!("{err}");
                    sleep(Duration::from_secs(5));
                    continue;
                }
            };

            last_timestamp = match candles.last() {
                None => {
                    break;
                }
                Some(last) => {
                    if last_timestamp == last.timestamp {
                        break;
                    } else {
                        last.timestamp
                    }
                }
            };

            if is_full_sync {
                let last_time = DateTime::from_timestamp_millis(last_timestamp)
                    .unwrap()
                    .to_rfc3339();
                info!("Last time {last_time}");
            }

            database.upsert_candles(candles);

            sleep(Duration::from_millis(300));
        }

        info!("Sync done");
    }
}
