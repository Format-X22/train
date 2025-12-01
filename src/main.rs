mod candle;
mod config;
mod dto;
mod simulator;
mod stock;
mod trader;

use crate::config::{Config, ENV_PREFIX};
use crate::simulator::runner::run_simulation;
use crate::stock::Stock;
use crate::trader::Trader;
use dotenvy::dotenv;
use env_logger::{Builder, Target};
use log::{LevelFilter, info};

fn main() {
    let config = parse_envs();

    init_logs(config.log_level);

    info!("Boot...");

    let stock = Stock::new(config.public_key, config.private_key);
    
    if config.is_simulation {
        run_simulation(
            stock,
            config.ticker,
            config.order_size,
            config.padding_percent,
            config.order_decimals,
        )
    } else {
        let mut trader = Trader::new(
            stock,
            config.ticker,
            config.order_size,
            config.padding_percent,
            config.order_decimals,
        );

        trader.trade();
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
