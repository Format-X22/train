use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use reqwest::blocking::{Client, RequestBuilder, Response};
use sha2::Sha256;
use snafu::Whatever;
use snafu::prelude::*;
use std::collections::HashMap;
use crate::candle::Candle;
use crate::dto::{BalanceData, BasicResponse, CandlesData};

type HmacSha256 = Hmac<Sha256>;

const RECV_WINDOW: &str = "5000";
const API: &str = "https://api.bybit.com/v5";

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

    pub fn get_balance(&self) -> Result<f64, Whatever> {
        let point = "account/wallet-balance";
        let params = HashMap::from([("accountType", "UNIFIED")]);
        let response = self.call_with_get(point, params)?;
        let parsed = response
            .json::<BalanceData>()
            .whatever_context("On parse balance data")?;
        let data = self.check_and_extract_data(parsed)?;

        match data.list.first() {
            None => whatever!("No data in balance response"),
            Some(balance) => Ok(balance.total_available_balance),
        }
    }

    pub fn get_candles(&self, from: i64) -> Result<Vec<Candle>, Whatever> {
        let point = "market/kline";
        let from = from.to_string();
        let params = HashMap::from([
            ("category", "linear"),
            ("symbol", "BTCUSDT"),
            ("interval", "60"),
            ("start", &from),
            ("limit", "1000"),
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
