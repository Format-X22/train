mod candle;
mod config;
mod database;
mod deal;
mod dto;
mod simulator;
mod stock;
mod trade_state;
mod trader;

use crate::candle::Candle;
use crate::config::{Config, ENV_PREFIX};
use crate::database::Database;
use crate::simulator::simulate;
use crate::stock::Stock;
use crate::trade_state::TradeState;
use crate::trader::trade;
use dotenvy::dotenv;
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let config = parse_envs();

    init_logs(config.log_level);
    info!("Boot...");

    let database = Database::new();
    let stock = Stock::new(config.public_key, config.private_key);

    if config.is_simulator {
        simulate(
            &database,
            &stock,
            config.padding_percent,
            config.stop_percent,
            config.capital_percent,
        );
    } else {
        trade(&database, &stock);
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
