use crate::candle::Candle;
use crate::dto::Order;
use crate::simulator::database;
use crate::simulator::position::Position;
use crate::stock::Side;
use crate::trader::Trader;
use chrono::NaiveDate;
use log::info;
use std::collections::HashMap;

pub struct Simulator {
    capital: f64,

    trader: Trader,
    simulate_from_ms: i64,

    position: Option<Position>,
    sell_orders: HashMap<String, Order>,
    buy_orders: HashMap<String, Order>,
}

impl Simulator {
    pub fn new(trader: Trader, simulate_from: String) -> Self {
        Self {
            capital: 100.0,
            trader,
            simulate_from_ms: NaiveDate::parse_from_str(&simulate_from, "%Y-%m-%d")
                .expect("Invalid 'simulate from' date")
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_millis(),
            position: None,
            sell_orders: HashMap::new(),
            buy_orders: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let candles = self.get_candles();

        info!("{} candles in simulation", candles.len());

        for candle in candles {
            self.create_orders(&candle);
            self.handle_collision(&candle);

            // TODO Check collisions
            // TODO Check inline collisions
            // TODO Place new orders
            // TODO Calc orders max count, leverage, liquidation price
        }

        println!("{} {}", self.buy_orders.len(), self.sell_orders.len());

        for order in self.sell_orders.values() {
            println!("{} {}", order.price, order.qty);
        }
        println!("----------");
        for order in self.buy_orders.values() {
            println!("{} {}", order.price, order.qty);
        }

        // TODO Print results
    }

    fn handle_collision(&mut self, candle: &Candle) {
        let mut to_remove_sells = Vec::new();
        for order in self.sell_orders.values() {
            if candle.high > order.price {
                to_remove_sells.push(order.order_id.clone())

                // TODO -
            }
        }
        for id in to_remove_sells {
            self.sell_orders.remove(&id);
        }

        let mut to_remove_buys = Vec::new();
        for order in self.buy_orders.values() {
            if candle.low < order.price {
                to_remove_buys.push(order.order_id.clone())

                // TODO -
            }
        }
        for id in to_remove_buys {
            self.buy_orders.remove(&id);
        }
    }

    fn create_orders(&mut self, candle: &Candle) {
        let mut current_orders = Vec::new();

        self.sell_orders
            .values()
            .for_each(|i| current_orders.push(i.clone()));
        self.buy_orders
            .values()
            .for_each(|i| current_orders.push(i.clone()));

        let deal = self.trader.calc_deal_values(candle.open, &current_orders, self.capital);

        let id = candle.timestamp.to_string();
        let sell_order = Order {
            order_id: id.clone(),
            side: Side::Sell,
            price: deal.sell,
            qty: deal.qty,
        };
        let buy_order = Order {
            order_id: id.clone(),
            side: Side::Buy,
            price: deal.buy,
            qty: deal.qty,
        };

        self.sell_orders.insert(id.clone(), sell_order);
        self.buy_orders.insert(id.clone(), buy_order);
    }

    fn get_candles(&self) -> Vec<Candle> {
        let database = database::Database::new(&self.trader.ticker);

        info!("Syncing candles...");
        Candle::sync(
            &database,
            &self.trader.stock,
            &self.trader.ticker,
            self.trader.candle_size,
        );
        info!("Sync completed");

        Candle::get_candles(&database, self.simulate_from_ms)
    }
}
