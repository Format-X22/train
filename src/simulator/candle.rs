use std::thread::sleep;
use std::time::Duration;
use chrono::DateTime;
use log::{error, info};
use crate::candle::Candle;
use crate::simulator::database::Database;
use crate::stock::Stock;

const SYNC_FROM: &str = "2021-06-01T00:00:00-00:00";

impl Candle {
    pub fn sync(database: &Database, stock: &Stock, ticker: &str) {
        let mut last_timestamp = Candle::get_last_candle_timestamp(database);
        let mut is_full_sync = false;

        if last_timestamp == 0 {
            info!("Full sync required, start...");
            let sync_from = DateTime::parse_from_rfc3339(SYNC_FROM).unwrap();
            last_timestamp = sync_from.timestamp_millis();
            is_full_sync = true;
        }

        loop {
            let candles = match stock.get_candles_from(last_timestamp - 1, ticker) {
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

            Candle::upsert_candles(database, candles);

            sleep(Duration::from_millis(300));
        }
    }

    pub fn get_candles(database: &Database, limit: i32) -> Vec<Candle> {
        let query = format!("SELECT * FROM `candles` ORDER BY `timestamp` DESC LIMIT {limit}");

        let mut candles: Vec<Candle> = database
            .get(query)
            .iter()
            .map(|row| Candle {
                timestamp: row.read("timestamp"),
                open: row.read("open"),
                high: row.read("high"),
                low: row.read("low"),
                close: row.read("close"),
            })
            .collect();

        candles.reverse();
        candles
    }
    
    fn get_last_candle_timestamp(database: &Database) -> i64 {
        let query =
            "SELECT `timestamp` FROM `candles` ORDER BY `timestamp` DESC LIMIT 1".to_string();
        let result = database.get(query);

        match result.first() {
            None => 0,
            Some(row) => row.read("timestamp"),
        }
    }

    fn upsert_candles(database: &Database, candles: Vec<Candle>) {
        let mut rows: Vec<String> = vec![];

        rows.push("BEGIN TRANSACTION;".to_string());

        for candle in candles {
            rows.push(
                format!(
                    "INSERT OR REPLACE INTO `candles` VALUES\
                    (\"{}\",\"{}\",\"{}\",\"{}\",\"{}\");",
                    candle.timestamp, candle.open, candle.high, candle.low, candle.close
                )
                    .to_string(),
            );
        }
        rows.push("COMMIT;".to_string());

        database.update(rows.join("\n"));
    }
}
