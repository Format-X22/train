use crate::database::Database;
use log::info;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Copy, Clone, Display, EnumString)]
pub enum DealStatus {
    Initial,
    FilledBuy,
    FilledSell,
    Profit,
    FailBuy,
    FailSell,
}

pub struct Deal {
    pub timestamp: i64,
    pub status: DealStatus,
    pub buy_order_id: String,
    pub sell_order_id: String,
    pub amount: f64,
    pub unfilled_amount: f64,
    pub base_price: f64,
    pub buy_price: f64,
    pub buy_stop_price: f64,
    pub sell_price: f64,
    pub sell_stop_price: f64,
}

impl Deal {
    pub fn save_to_database(&self, database: &Database) {
        let query = format!(
            "INSERT OR REPLACE INTO `deals` VALUES\
            (\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\");",
            self.timestamp,
            self.status,
            self.buy_order_id,
            self.sell_order_id,
            self.amount,
            self.unfilled_amount,
            self.base_price,
            self.buy_price,
            self.buy_stop_price,
            self.sell_price,
            self.sell_stop_price,
        );

        database.update(query);
    }

    pub fn get_opened(database: &Database) -> Vec<Deal> {
        let query = format!(
            "SELECT * FROM `deals` \
                WHERE `status` IN(\"{}\",\"{}\",\"{}\") \
                ORDER BY `timestamp` ASC;\
            ",
            DealStatus::Initial,
            DealStatus::FilledBuy,
            DealStatus::FilledSell,
        );

        let result = database.get(query);
        let converted = result.iter().map(|row| Deal {
            timestamp: row.read("timestamp"),
            status: DealStatus::from_str(row.read("status")).expect("On parse status"),
            buy_order_id: row.read::<&str, _>("buy_order_id").to_string(),
            sell_order_id: row.read::<&str, _>("sell_order_id").to_string(),
            amount: row.read("amount"),
            unfilled_amount: row.read("unfilled_amount"),
            base_price: row.read("base_price"),
            buy_price: row.read("buy_price"),
            buy_stop_price: row.read("buy_stop_price"),
            sell_price: row.read("sell_price"),
            sell_stop_price: row.read("sell_stop_price"),
        });
        
        converted.collect()
    }

    pub fn truncate(database: &Database) {
        let query = "DELETE FROM `deals`".to_string();

        database.update(query);
    }
}
