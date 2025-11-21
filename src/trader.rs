use crate::candle::Candle;
use crate::stock::{Side, Stock};
use chrono::Utc;
use log::{error, info};
use std::thread::sleep;
use std::time::Duration;

pub struct Trader {
    stock: Stock,
    ticker: String,
    padding_percent: f64,
    order_size: f64,
    order_decimals: usize,
    last_candle_timestamp: i64,
}

impl Trader {
    pub fn new(
        stock: Stock,
        ticker: String,
        order_size: f64,
        padding_percent: f64,
        order_decimals: usize,
    ) -> Self {
        Self {
            stock,
            ticker,
            padding_percent,
            order_size,
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

            self.place_order(Side::Buy, buy_price);
            self.place_order(Side::Sell, sell_price);

            info!("New orders placed");

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

    fn place_order(&self, side: Side, price: f64) {
        let amount = self.order_size;

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
}
