use crate::database::Database;
use crate::stock::Stock;
use log::info;
use sqlite::Row;

pub struct TradeState {
    pub timestamp: i64,
    pub trade_capital: f64,
    pub available_capital: f64,
    pub awaited_deals: i64,
    pub stuck_deals: i64,
}

impl TradeState {
    pub fn save_to_database(&self, database: &Database) {
        let query = format!(
            "INSERT OR REPLACE INTO `trade_states` VALUES\
            (\"{}\",\"{}\",\"{}\",\"{}\",\"{}\");",
            self.timestamp,
            self.trade_capital,
            self.available_capital,
            self.awaited_deals,
            self.stuck_deals,
        );

        database.update(query);
    }

    pub fn initiate(database: &Database, stock: &Stock, last_candle_timestamp: i64) {
        let query = "SELECT COUNT(*) FROM `trade_states`;".to_string();
        let result = database.get(query);
        let count: i64 = result.first().expect("On initiate on count").read(0);

        if count == 0 {
            let capital = stock.get_balance().unwrap();
            let first_state = TradeState {
                timestamp: last_candle_timestamp,
                trade_capital: capital,
                available_capital: capital,
                awaited_deals: 0,
                stuck_deals: 0,
            };
            first_state.save_to_database(database);
        }
    }

    pub fn get_last_state(database: &Database) -> TradeState {
        let query = "SELECT * FROM `trade_states` ORDER BY `timestamp` DESC LIMIT 1;".to_string();
        let result = database.get(query);
        let row = result.first().expect("Empty trade_states collection");

        TradeState {
            timestamp: row.read("timestamp"),
            trade_capital: row.read("trade_capital"),
            available_capital: row.read("available_capital"),
            awaited_deals: row.read("awaited_deals"),
            stuck_deals: row.read("stuck_deals"),
        }
    }

    pub fn truncate(database: &Database) {
        let query = "DELETE FROM `trade_states`".to_string();

        database.update(query);
    }
}
