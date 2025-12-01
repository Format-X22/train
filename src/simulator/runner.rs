use crate::candle::Candle;
use crate::simulator::database;
use crate::stock::Stock;
use log::info;

pub fn run_simulation(
    stock: Stock,
    ticker: String,
    order_size: f64,
    padding_percent: f64,
    order_decimals: usize,
) {
    let candles = get_candles(&stock, &ticker);

    for candle in candles {
        // TODO Check collisions
        // TODO Check inline collisions
        // TODO Place new orders
        // TODO Calc orders max count, leverage, liquidation price
    }

    // TODO Print results
}

fn get_candles(stock: &Stock, ticker: &str) -> Vec<Candle> {
    let database = database::Database::new(&ticker);

    info!("Syncing candles...");
    Candle::sync(&database, &stock, &ticker);
    info!("Sync completed");

    Candle::get_candles(&database, 100_000)
}
