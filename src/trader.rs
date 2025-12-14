use crate::candle::Candle;
use crate::dto::OrdersCountBySide;
use crate::stock::{Side, Stock};
use chrono::Utc;
use log::{error, info};
use std::thread::sleep;
use std::time::Duration;

pub struct Trader {
    stock: Stock,
    ticker: String,
    padding_percent: f64,
    capital_percent: f64,
    minimum_size: f64,
    order_decimals: usize,
    last_candle_timestamp: i64,
}

impl Trader {
    pub fn new(
        stock: Stock,
        ticker: String,
        padding_percent: f64,
        capital_percent: f64,
        order_decimals: usize,
    ) -> Self {
        let pow_back = -(order_decimals as i32);
        let minimum_size = 1.0 * 10.0_f64.powi(pow_back);

        Self {
            stock,
            ticker,
            padding_percent,
            capital_percent,
            minimum_size,
            order_decimals,
            last_candle_timestamp: Utc::now().timestamp_millis(),
        }
    }

    pub fn trade(&mut self) {
        loop {
            let candle = self.get_new_candle();
            let base_price = candle.open;
            let padding_size = base_price * (self.padding_percent / 100.0);
            let buy_price = base_price - padding_size;
            let sell_price = base_price + padding_size;
            let balance = self.get_balance();
            let capital_for_order = balance * (self.capital_percent / 100.0);
            let base_amount = capital_for_order / base_price;
            let current_orders_count = self.get_orders_count_by_side();
            let waited_count = i32::max(current_orders_count.buy, current_orders_count.sell);
            let waited_mul = 1.0 - (self.capital_percent / 100.0);
            let fact_amount = base_amount * (waited_mul.powi(waited_count));
            let amount = f64::max(fact_amount, self.minimum_size);

            self.place_order(Side::Buy, buy_price, amount);
            self.place_order(Side::Sell, sell_price, amount);

            if amount > self.minimum_size {
                info!("New orders placed");
            } else {
                info!("New minimal orders placed");
            }

            sleep(Duration::from_secs(5));
        }
    }

    fn get_new_candle(&mut self) -> Candle {
        loop {
            match self.stock.get_candles(&self.ticker) {
                Ok(candles) => match candles.last() {
                    Some(candle) => {
                        if candle.timestamp > self.last_candle_timestamp {
                            self.last_candle_timestamp = candle.timestamp;

                            return *candle;
                        }
                    }
                    None => {
                        error!("Empty candles list")
                    }
                },
                Err(error) => {
                    error!("Problem with load candles - {error}")
                }
            }
            sleep(Duration::from_secs(5));
        }
    }

    fn place_order(&self, side: Side, price: f64, amount: f64) {
        loop {
            match self
                .stock
                .place_order(&self.ticker, side, price, self.order_decimals, amount)
            {
                Ok(_) => return (),
                Err(error) => {
                    error!("Problem with place order - {side} {price} {amount} - {error}")
                }
            }
            sleep(Duration::from_secs(5));
        }
    }

    fn get_balance(&self) -> f64 {
        loop {
            match self.stock.get_balance() {
                Ok(balance) => return balance,
                Err(error) => {
                    error!("Problem with get balance - {error}")
                }
            }
            sleep(Duration::from_secs(5));
        }
    }

    fn get_orders_count_by_side(&self) -> OrdersCountBySide {
        loop {
            match self.stock.get_orders_count_by_side(&self.ticker) {
                Ok(data) => return data,
                Err(error) => {
                    error!("Problem with get orders counts - {error}")
                }
            }
            sleep(Duration::from_secs(5));
        }
    }
}
