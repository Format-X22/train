use crate::config::{Config, ENV_PREFIX};
use crate::data::candle_sync::CandleSync;
use crate::data::database::Database;
use crate::trade::stock::Stock;
use dotenvy::dotenv;
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use std::cell::RefCell;

mod config;
mod data;
mod trade;

fn main() {
    let config = parse_envs();

    init_logs(config.log_level);
    info!("Boot...");

    let database = Database::new();
    let database_cell = RefCell::new(database);
    let stock = Stock::new(config.public_key, config.private_key);
    let stock_cell = RefCell::new(stock);
    let candle_sync = CandleSync::new(database_cell, stock_cell);

    candle_sync.sync();

    if config.is_simulator {
        info!("Start simulator...");
        // TODO simulate
    } else {
        info!("Start trading...");
        // TODO run
    }
}

fn parse_envs() -> Config {
    dotenv().expect("On init envs");
    envy::prefixed(ENV_PREFIX)
        .from_env::<Config>()
        .expect("On parse envs")
}

fn init_logs(level: LevelFilter) {
    let mut logs_builder = Builder::new();

    logs_builder.filter_level(level);
    logs_builder.target(Target::Stdout);
    logs_builder.init();
}
