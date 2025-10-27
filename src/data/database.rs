use crate::data::candle::Candle;
use sqlite::Connection;
use std::string::ToString;
use strum_macros::Display;

#[derive(Display)]
pub enum Order {
    ASC,
    DESC,
}

const MIGRATION: &str = "
    CREATE TABLE IF NOT EXISTS `candles`(
        `timestamp` INTEGER PRIMARY KEY ASC,
        `open` REAL NOT NULL,
        `high` REAL NOT NULL,
        `low` REAL NOT NULL,
        `close` REAL NOT NULL
    );
";

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Database {
        let path = "db.sqlite".to_string();
        let error_message = format!("Fatal on connect to {path}");
        let connection = sqlite::open(path).expect(&error_message);

        connection
            .execute(MIGRATION.to_string())
            .expect("Fatal on migrate");

        Database { connection }
    }

    pub fn get_last_candle_timestamp(&self) -> i64 {
        let query =
            "SELECT `timestamp` FROM `candles` ORDER BY `timestamp` DESC LIMIT 1".to_string();
        let result = self
            .connection
            .prepare(query)
            .expect("On build query for first candle timestamp")
            .into_iter()
            .map(|row| row.expect("On get data from first candle timestamp"))
            .next();

        match result {
            None => 0,
            Some(row) => row.read::<i64, _>("timestamp"),
        }
    }

    pub fn upsert_candles(&self, candles: Vec<Candle>) {
        let start = "BEGIN TRANSACTION;".to_string();
        let end = "COMMIT;".to_string();
        let mut rows: Vec<String> = vec![];

        rows.push(start);

        for candle in candles {
            rows.push(
                format!(
                    "INSERT OR REPLACE INTO `candles` VALUES ( \
                    \"{}\",\
                    \"{}\",\
                    \"{}\",\
                    \"{}\",\
                    \"{}\"\
                    );",
                    candle.timestamp, candle.open, candle.high, candle.low, candle.close
                )
                .to_string(),
            );
        }

        rows.push(end);

        let query = rows.join("\n");

        self.connection
            .execute(query)
            .expect("Fatal on insert candles");
    }

    pub fn get_candles(&self, limit: i32) -> Vec<Candle> {
        let query =
            format!("SELECT * FROM `candles` ORDER BY `timestamp` DESC LIMIT {limit}");

        let mut candles: Vec<Candle> = self.connection
            .prepare(query)
            .expect("On build query for candles list")
            .into_iter()
            .map(|row| {
                let raw_row = row.expect("On get data from candles list");

                Candle {
                    timestamp: raw_row.read::<i64, _>("timestamp"),
                    open: raw_row.read::<f64, _>("open"),
                    high: raw_row.read::<f64, _>("high"),
                    low: raw_row.read::<f64, _>("low"),
                    close: raw_row.read::<f64, _>("close"),
                }
            })
            .collect();
        
        candles.reverse();
        candles
    }
}
