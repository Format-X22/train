use crate::candle::Candle;
use crate::database::Database;
use crate::stock::Stock;
use crate::trade_state::TradeState;
use std::thread::sleep;
use std::time::Duration;

pub fn trade(database: &Database, stock: &Stock) {
    let last_candle_timestamp = Candle::get_last_candle_timestamp(&database);

    TradeState::initiate(&database, &stock, last_candle_timestamp);

    loop {
        Candle::sync(&database, &stock);
        // TODO Trade
        sleep(Duration::from_secs(5));
    }
}
