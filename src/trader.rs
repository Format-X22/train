use crate::candle::Candle;
use crate::dto::OrdersCountBySide;
use crate::stock::{Side, Stock};
use crate::{repeat_each_ms, with_retry};
use chrono::Utc;
use log::{error, info};
use std::thread::sleep;
use std::time::Duration;

const RETRY_MS: u64 = 500;
const TRADE_LOOP_MS: u64 = 100;

pub struct Trader {
    stock: Stock,
    ticker: String,
    padding_percent: f64,
    capital_percent: f64,
    minimum_size: f64,
    order_decimals: usize,
    price_decimals: usize,
    last_candle_timestamp: i64,
}

impl Trader {
    pub fn new(
        stock: Stock,
        ticker: String,
        padding_percent: f64,
        capital_percent: f64,
        order_decimals: usize,
        price_decimals: usize,
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
            price_decimals,
            last_candle_timestamp: Utc::now().timestamp_millis(),
        }
    }

    pub fn trade(&mut self) {
        repeat_each_ms!(TRADE_LOOP_MS, {
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
        })
    }

    fn get_new_candle(&mut self) -> Candle {
        repeat_each_ms!(
            RETRY_MS,
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
        )
    }

    fn place_order(&self, side: Side, price: f64, amount: f64) {
        with_retry!(
            RETRY_MS,
            self.stock.place_order(
                &self.ticker,
                side,
                price,
                self.order_decimals,
                self.price_decimals,
                amount,
            ),
            "Problem with place order"
        )
    }

    fn get_balance(&self) -> f64 {
        with_retry!(
            RETRY_MS,
            self.stock.get_balance(),
            "Problem with get balance"
        )
    }

    fn get_orders_count_by_side(&self) -> OrdersCountBySide {
        with_retry!(
            RETRY_MS,
            self.stock.get_orders_count_by_side(&self.ticker),
            "Problem with get orders counts"
        )
    }
}
