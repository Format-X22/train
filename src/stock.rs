use crate::candle::Candle;
use crate::dto::{
    BalanceData, BasicResponse, CancelOrderData, CandlesData, LiquidateData, Order, OrdersData,
    PlaceOrderData,
};
use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use reqwest::blocking::{Client, RequestBuilder, Response};
use serde::Deserialize;
use sha2::Sha256;
use snafu::Whatever;
use snafu::prelude::*;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use strum_macros::Display;

type HmacSha256 = Hmac<Sha256>;

const RECV_WINDOW: &str = "5000";
const API: &str = "https://api.bybit.com/v5";

#[derive(Display, Copy, Clone, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

pub struct Stock {
    public_key: String,
    private_key: String,
    client: Client,
}

impl Stock {
    pub fn new(public_key: String, private_key: String) -> Self {
        Self {
            client: Client::new(),
            public_key,
            private_key,
        }
    }

    pub fn get_candles(&self, ticker: &str, candle_size: i64) -> Result<Vec<Candle>, Whatever> {
        let point = "market/kline";
        let interval = candle_size.to_string();
        let params = HashMap::from([
            ("category", "linear"),
            ("symbol", ticker),
            ("interval", &interval),
            ("limit", "10"),
        ]);

        let response = self.call_with_get(point, params)?;
        let parsed = response
            .json::<CandlesData>()
            .whatever_context("On parse candles data")?;
        let raw = self.check_and_extract_data(parsed)?;
        let mut data = vec![];

        for raw in raw.list {
            data.push(Candle::from_raw(raw)?)
        }
        data.reverse();

        Ok(data)
    }

    pub fn get_balance(&self) -> Result<f64, Whatever> {
        let point = "account/wallet-balance";
        let params = HashMap::from([("accountType", "UNIFIED")]);
        let response = self.call_with_get(point, params)?;
        let parsed = response
            .json::<BalanceData>()
            .whatever_context("On parse json for balance")?;
        let data = self.check_and_extract_data(parsed)?;

        match data.list.first() {
            None => whatever!("Empty balance response!"),
            Some(balance) => Ok(balance.total_margin_balance),
        }
    }

    pub fn get_orders(&self, ticker: &str) -> Result<Vec<Order>, Whatever> {
        let mut orders = Vec::new();
        let mut cursor = "".to_string();

        loop {
            let point = "order/realtime";
            let params = HashMap::from([
                ("category", "linear"),
                ("symbol", ticker),
                ("openOnly", "0"),
                ("orderFilter", "Order"),
                ("cursor", &cursor),
                ("limit", "50"),
            ]);
            let response = self.call_with_get(point, params)?;
            let parsed = response
                .json::<OrdersData>()
                .whatever_context("On parse json for balance")?;
            let data = self.check_and_extract_data(parsed)?;

            cursor = data.next_page_cursor;

            for order in data.list {
                orders.push(order);
            }

            if cursor.len() == 0 {
                return Ok(orders);
            }
            sleep(Duration::from_millis(10));
        }
    }

    pub fn place_order(
        &self,
        ticker: &str,
        side: Side,
        price: f64,
        order_decimals: usize,
        price_decimals: usize,
        qty: f64,
    ) -> Result<(), Whatever> {
        let point = "order/create";
        let qty = format!("{:.1$}", qty, order_decimals);
        let price = format!("{:.1$}", price, price_decimals);
        let side = side.to_string();
        let order_params = HashMap::from([
            ("category", "linear"),
            ("symbol", ticker),
            ("side", &side),
            ("orderType", "Limit"),
            ("qty", &qty),
            ("price", &price),
        ]);

        let main_response = self.call_with_post(point, order_params)?;
        let main_parsed = main_response
            .json::<PlaceOrderData>()
            .whatever_context("On parse json for place order")?;

        self.check_and_extract_data(main_parsed)?;

        Ok(())
    }

    pub fn liquidate(&self, ticker: &str, order_decimals: usize, qty: f64) -> Result<(), Whatever> {
        let point = "order/create";
        let qty = format!("{:.1$}", qty, order_decimals);
        let side = Side::Buy.to_string();
        let order_params = HashMap::from([
            ("category", "linear"),
            ("symbol", ticker),
            ("side", &side),
            ("orderType", "Market"),
            ("qty", &qty),
        ]);

        let main_response = self.call_with_post(point, order_params)?;
        let main_parsed = main_response
            .json::<LiquidateData>()
            .whatever_context("On parse json for place order")?;

        self.check_and_extract_data(main_parsed)?;

        Ok(())
    }

    pub fn cancel_order(&self, ticker: &str, order_id: &str) -> Result<(), Whatever> {
        let point = "order/cancel";
        let order_params = HashMap::from([
            ("category", "linear"),
            ("symbol", ticker),
            ("orderId", order_id),
        ]);

        let main_response = self.call_with_post(point, order_params)?;
        let main_parsed = main_response
            .json::<CancelOrderData>()
            .whatever_context("On parse json for place order")?;

        self.check_and_extract_data(main_parsed)?;

        Ok(())
    }

    fn call_with_get(
        &self,
        point: &str,
        params: HashMap<&str, &str>,
    ) -> Result<Response, Whatever> {
        let timestamp = Utc::now().timestamp_millis();
        let query = self.generate_query_str(&params);
        let signature = self.generate_signature(timestamp, query.clone())?;
        let path = format!("{API}/{point}?{query}");
        let builder = self.client.get(path.clone());
        let response = self
            .inject_api_headers(builder, &signature, timestamp)
            .send()
            .whatever_context(format!("On GET request to {path}"))?;

        self.handle_status(response)
    }

    fn call_with_post(
        &self,
        point: &str,
        params: HashMap<&str, &str>,
    ) -> Result<Response, Whatever> {
        let timestamp = Utc::now().timestamp_millis();
        let signature = self.generate_signature(
            timestamp,
            serde_json::to_string(&params).whatever_context("On pack JSON for POST request")?,
        )?;
        let path = format!("{API}/{point}");
        let builder = self.client.post(path.clone()).json(&params);
        let response = self
            .inject_api_headers(builder, &signature, timestamp)
            .send()
            .whatever_context(format!("On POST request to {path}"))?;

        self.handle_status(response)
    }

    fn generate_signature(&self, timestamp: i64, params: String) -> Result<String, Whatever> {
        let mut mac =
            HmacSha256::new_from_slice(self.private_key.as_bytes()).whatever_context("On HMAC")?;

        mac.update(timestamp.to_string().as_bytes());
        mac.update(self.public_key.as_bytes());
        mac.update(RECV_WINDOW.as_bytes());
        mac.update(params.as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        Ok(hex::encode(code_bytes))
    }

    fn generate_query_str(&self, params: &HashMap<&str, &str>) -> String {
        params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<String>>()
            .join("&")
    }

    fn inject_api_headers(
        &self,
        builder: RequestBuilder,
        signature: &String,
        timestamp: i64,
    ) -> RequestBuilder {
        builder
            .header("X-BAPI-API-KEY", &self.public_key)
            .header("X-BAPI-SIGN", signature)
            .header("X-BAPI-SIGN-TYPE", "2")
            .header("X-BAPI-TIMESTAMP", timestamp)
            .header("X-BAPI-RECV-WINDOW", RECV_WINDOW)
    }

    fn handle_status(&self, response: Response) -> Result<Response, Whatever> {
        if response.status().is_success() {
            Ok(response)
        } else {
            whatever!("{}", response.status())
        }
    }

    fn check_and_extract_data<T>(&self, parsed: BasicResponse<T>) -> Result<T, Whatever> {
        if parsed.ret_code == 0 {
            Ok(parsed.result)
        } else {
            whatever!("{}", parsed.ret_msg)
        }
    }
}
