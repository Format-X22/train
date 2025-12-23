use crate::candle::Candle;
use crate::dto::Order;
use crate::stock::{Side, Stock};
use crate::{repeat_each_ms, with_retry};
use chrono::Utc;
use log::{error, info};
use std::thread::sleep;
use std::time::Duration;

const RETRY_MS: u64 = 500;
const TRADE_LOOP_MS: u64 = 100;

struct DealValues {
    pub buy: f64,
    pub sell: f64,
    pub qty: f64,
}

pub struct Trader {
    stock: Stock,
    ticker: String,
    padding_percent: f64,
    capital_percent: f64,
    candle_size: i64,
    risk_deduction: f64,
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
        candle_size: i64,
        risk_deduction: f64,
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
            candle_size,
            risk_deduction,
            minimum_size,
            order_decimals,
            price_decimals,
            last_candle_timestamp: Utc::now().timestamp_millis(),
        }
    }

    pub fn trade(&mut self) {
        repeat_each_ms!(TRADE_LOOP_MS, {
            let candle = self.get_new_candle();
            let orders = self.get_orders();
            let deal_values = self.calc_deal_values(candle.open, &orders);

            self.place_order(Side::Buy, deal_values.buy, deal_values.qty);
            self.place_order(Side::Sell, deal_values.sell, deal_values.qty);

            if deal_values.qty > self.minimum_size {
                info!("New orders placed");
            } else {
                info!("New minimal orders placed");
            }

            self.liquidate_dangling_orders(candle.open, orders);
        })
    }

    fn calc_deal_values(&mut self, base_price: f64, orders: &Vec<Order>) -> DealValues {
        let padding_size = base_price * (self.padding_percent / 100.0);
        let buy = base_price - padding_size;
        let sell = base_price + padding_size;
        let balance = self.get_balance();
        let capital_for_order = balance * (self.capital_percent / 100.0);
        let base_amount = capital_for_order / base_price;
        let mut buy_count = 0;
        let mut sell_count = 0;

        for order in orders {
            match order.side {
                Side::Buy => buy_count += 1,
                Side::Sell => sell_count += 1,
            }
        }

        let waited_count = i32::max(buy_count, sell_count);
        let waited_power = (waited_count as f64 * self.risk_deduction).trunc() as i32;
        let waited_mul = 1.0 - (self.capital_percent / 100.0);
        let fact_qty = base_amount * (waited_mul.powi(waited_power));
        let qty = f64::max(fact_qty, self.minimum_size);

        DealValues { buy, sell, qty }
    }

    fn liquidate_dangling_orders(&mut self, base_price: f64, orders: Vec<Order>) {
        for order in orders {
            if let Side::Buy = order.side {
                if order.price * 2.0 < base_price {
                    info!("Liquidate {}", order.price);
                    self.liquidate(order.qty);
                    self.cancel_order(order.order_id);
                }
            }
        }
    }

    fn get_new_candle(&mut self) -> Candle {
        repeat_each_ms!(
            RETRY_MS,
            match self.stock.get_candles(&self.ticker, self.candle_size) {
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

    fn place_order(&self, side: Side, price: f64, qty: f64) {
        with_retry!(
            RETRY_MS,
            self.stock.place_order(
                &self.ticker,
                side,
                price,
                self.order_decimals,
                self.price_decimals,
                qty,
            ),
            "Problem with place order"
        )
    }

    fn liquidate(&self, qty: f64) {
        with_retry!(
            RETRY_MS,
            self.stock.liquidate(&self.ticker, self.order_decimals, qty),
            "Problem with liquidate"
        )
    }

    fn cancel_order(&self, order_id: String) {
        with_retry!(
            RETRY_MS,
            self.stock.cancel_order(&self.ticker, &order_id),
            "Problem with cancel order"
        )
    }

    fn get_balance(&self) -> f64 {
        with_retry!(
            RETRY_MS,
            self.stock.get_balance(),
            "Problem with get balance"
        )
    }

    fn get_orders(&self) -> Vec<Order> {
        with_retry!(
            RETRY_MS,
            self.stock.get_orders(&self.ticker),
            "Problem with get orders counts"
        )
    }
}
