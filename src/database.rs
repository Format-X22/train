use sqlite::{Connection, Row};
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
    CREATE TABLE IF NOT EXISTS `deals`(
        `timestamp` INTEGER PRIMARY KEY ASC,
        `status` TEXT NOT NULL,
        `buy_order_id` TEXT NOT NULL,
        `sell_order_id` TEXT NOT NULL,
        `amount` REAL NOT NULL,
        `unfilled_amount` REAL NOT NULL,
        `base_price` REAL NOT NULL,
        `buy_price` REAL NOT NULL,
        `buy_stop_price` REAL NOT NULL,
        `sell_price` REAL NOT NULL,
        `sell_stop_price` REAL NOT NULL
    );
    CREATE TABLE IF NOT EXISTS `trade_states`(
        `timestamp` INTEGER PRIMARY KEY ASC,
        `trade_capital` REAL NOT NULL,
        `available_capital` REAL NOT NULL,
        `awaited_deals` INTEGER NOT NULL,
        `stuck_deals` INTEGER NOT NULL
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

    pub fn get(&self, query: String) -> Vec<Row> {
        let get_error = format!("On get - {}", &query);
        let parse_error = format!("On get on parse - {}", &query);

        self.connection
            .prepare(query)
            .expect(&get_error)
            .into_iter()
            .map(|row| row.expect(&parse_error))
            .collect()
    }

    pub fn update(&self, query: String) {
        let error = format!("On update - {}", &query);

        self.connection.execute(query).expect(&error)
    }
}
