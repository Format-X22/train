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
    max_stuck: usize,
    max_stuck_buy: usize,
    max_stuck_sell: usize,
    drop_count: i64,

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
            max_stuck: 0,
            max_stuck_buy: 0,
            max_stuck_sell: 0,
            drop_count: 0,
        }
    }

    pub fn run(&mut self) {
        let candles = self.get_candles();

        info!("{} candles in simulation", candles.len());

        for candle in candles {
            self.create_orders(&candle);
            self.handle_collision(&candle);

            let current_stuck_buy = self.buy_orders.len();
            let current_stuck_sell = self.sell_orders.len();
            let current_stuck = self.buy_orders.len() + self.sell_orders.len();

            if current_stuck > self.max_stuck {
                self.max_stuck = current_stuck;

                if self.max_stuck == 4 {
                    println!(
                        "Date {} Buy {} Sell {}",
                        candle.timestamp,
                        self.buy_orders.len(),
                        self.sell_orders.len()
                    );
                }
            }

            if current_stuck_buy > self.max_stuck_buy {
                self.max_stuck_buy = current_stuck_buy
            }

            if current_stuck_sell > self.max_stuck_sell {
                self.max_stuck_sell = current_stuck_sell
            }

            // TODO Check collisions
            // TODO Check inline collisions
            // TODO Place new orders
            // TODO Calc orders max count, leverage, liquidation price
        }

        println!(
            "For now - Buy stuck {} Sell stuck {}",
            self.buy_orders.len(),
            self.sell_orders.len()
        );
        println!("Max stuck {}", self.max_stuck);
        println!("Max stuck buy {}", self.max_stuck_buy);
        println!("Max stuck sell {}", self.max_stuck_sell);
        println!("Drop count {}", self.drop_count);

        /*for order in self.sell_orders.values() {
            println!("{} {}", order.price, order.qty);
        }
        println!("----------");
        for order in self.buy_orders.values() {
            println!("{} {}", order.price, order.qty);
        }*/

        // TODO Print results
    }

    fn handle_collision(&mut self, candle: &Candle) {
        let mut to_remove_sells = Vec::new();
        for order in self.sell_orders.values() {
            if candle.high > order.price {
                to_remove_sells.push(order.order_id.clone());

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
            } else if candle.high > order.price * 2.0 {
                to_remove_buys.push(order.order_id.clone());
                self.drop_count += 1;
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

        let deal = self
            .trader
            .calc_deal_values(candle.open, &current_orders, self.capital);

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
            &self.trader.candle_size,
        );
        info!("Sync completed");

        Candle::get_candles(&database, self.simulate_from_ms)
    }
}
