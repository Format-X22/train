use sqlite::{Connection, Row};
use std::string::ToString;

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
    pub fn new(ticker: &str) -> Database {
        let path = format!("{}.sqlite", ticker);
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
