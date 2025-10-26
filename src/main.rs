use crate::config::{Config, ENV_PREFIX};
use dotenvy::dotenv;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use crate::data::database::Database;
use crate::trade::stock::Stock;

mod config;
mod trade;
mod data;

fn main() {
    let config = parse_envs();

    init_logs(config.log_level);

    let database = Database::new();
    
    // TODO sync candles

    if config.is_simulator {
        info!("Start simulator...");
        // TODO simulate
    } else {
        info!("Start trading...");
        
        let stock = Stock::new(config.public_key, config.private_key);
        
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
