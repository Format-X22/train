use crate::candle::Candle;
use crate::database::Database;
use crate::deal::{Deal, DealStatus};
use crate::stock::Stock;
use crate::trade_state::TradeState;
use chrono::{DateTime, Local};
use log::info;

const FEE: f64 = 0.036;

struct SimConfig {
    pub padding_percent: f64,
    pub stop_percent: f64,
    pub capital_percent: f64,
}

pub fn simulate(
    database: &Database,
    stock: &Stock,
    padding_percent: f64,
    stop_percent: f64,
    capital_percent: f64,
) {
    Candle::sync(&database, &stock);
    TradeState::truncate(database);
    Deal::truncate(database);

    let candles = Candle::get_candles(database, 100_000);
    let first_timestamp = candles.first().expect("Empty candles collection").timestamp;

    let first_trade_state = TradeState {
        timestamp: first_timestamp,
        trade_capital: 100.0,
        available_capital: 100.0,
        awaited_deals: 0,
        stuck_deals: 0,
    };
    first_trade_state.save_to_database(database);

    let first_date = DateTime::from_timestamp_millis(first_timestamp)
        .unwrap()
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M");

    info!("Start from {first_date}");

    let sim_config = SimConfig {
        padding_percent,
        stop_percent,
        capital_percent,
    };

    for candle in candles {
        handle_candle(database, candle, &sim_config);
    }

    // TODO Print stats

    info!("Done!");
}

fn handle_candle(database: &Database, candle: Candle, sim_config: &SimConfig) {
    let prev_trade_state = TradeState::get_last_state(database);
    let opened_deals = Deal::get_opened(database);
    let mut trade_state = TradeState {
        timestamp: candle.timestamp,
        trade_capital: prev_trade_state.trade_capital,
        available_capital: prev_trade_state.available_capital,
        awaited_deals: prev_trade_state.awaited_deals,
        stuck_deals: prev_trade_state.stuck_deals,
    };

    for mut deal in opened_deals {
        match deal.status {
            DealStatus::Initial => {
                if candle.high > deal.sell_stop_price {
                    handle_fail_sell(database, &mut deal, &mut trade_state, sim_config);
                    trade_state.awaited_deals -= 1;
                } else if candle.low < deal.buy_stop_price {
                    handle_fail_buy(database, &mut deal, &mut trade_state, sim_config);
                    trade_state.awaited_deals -= 1;
                } else if candle.high > deal.sell_price && candle.low < deal.buy_price {
                    handle_profit(database, &mut deal, &mut trade_state, sim_config);
                    trade_state.awaited_deals -= 1;
                } else if candle.high > deal.sell_price {
                    // TODO
                    trade_state.awaited_deals -= 1;
                    trade_state.stuck_deals += 1;
                } else if candle.low < deal.buy_price {
                    // TODO
                    trade_state.awaited_deals -= 1;
                    trade_state.stuck_deals += 1;
                } else {
                    // TODO
                }
            }
            DealStatus::FilledBuy => {
                if candle.low < deal.buy_stop_price {
                    handle_fail_buy(database, &mut deal, &mut trade_state, sim_config);
                    trade_state.stuck_deals -= 1;
                } else if candle.high > deal.sell_price {
                    handle_profit(database, &mut deal, &mut trade_state, sim_config);
                    trade_state.stuck_deals -= 1;
                } else {
                    // TODO
                }
            }
            DealStatus::FilledSell => {
                if candle.high > deal.sell_stop_price {
                    handle_fail_sell(database, &mut deal, &mut trade_state, sim_config);
                    trade_state.stuck_deals -= 1;
                } else if candle.low < deal.buy_price {
                    handle_profit(database, &mut deal, &mut trade_state, sim_config);
                    trade_state.stuck_deals -= 1;
                } else {
                    // TODO
                }
            }
            _ => {
                panic!("Ended deals in opened deals?")
            }
        }
    }

    trade_state.awaited_deals += 1;

    let capital_mul = sim_config.capital_percent / 100.0;
    let padding_mul = sim_config.padding_percent / 100.0;
    let stop_mul = sim_config.stop_percent / 100.0;
    let new_deal = Deal {
        status: DealStatus::Initial,
        timestamp: candle.timestamp,
        buy_order_id: "".to_string(),
        sell_order_id: "".to_string(),
        amount: trade_state.available_capital * capital_mul,
        unfilled_amount: 0.0,
        base_price: candle.close,
        buy_price: candle.close * (1.0 - padding_mul),
        buy_stop_price: candle.close * (1.0 - stop_mul),
        sell_price: candle.close * (1.0 + padding_mul),
        sell_stop_price: candle.close * (1.0 + stop_mul),
    };

    new_deal.save_to_database(database);
    trade_state.save_to_database(database);
}

fn handle_profit(
    database: &Database,
    deal: &mut Deal,
    trade_state: &mut TradeState,
    sim_config: &SimConfig,
) {
    let profit_mul = ((sim_config.padding_percent - FEE) / 100.0) * 2.0;
    let capital_mul = sim_config.capital_percent / 100.0;
    let stop_mul = sim_config.stop_percent / 100.0;
    let profit = deal.amount * (1.0 + profit_mul * capital_mul);
    let free = deal.amount * (1.0 + stop_mul);

    trade_state.trade_capital += profit;
    trade_state.available_capital += profit + free;

    deal.status = DealStatus::Profit;

    deal.save_to_database(database);
}

fn handle_fail_buy(
    database: &Database,
    deal: &mut Deal,
    trade_state: &mut TradeState,
    sim_config: &SimConfig,
) {
    //
}

fn handle_fail_sell(
    database: &Database,
    deal: &mut Deal,
    trade_state: &mut TradeState,
    sim_config: &SimConfig,
) {
    //
}
