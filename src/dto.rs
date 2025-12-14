use crate::candle::RawCandle;
use serde::Deserialize;
use serde_this_or_that::as_f64;
use crate::stock::Side;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicResponse<T> {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: T,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<T> {
    pub list: Vec<T>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyResponse {}

pub type PlaceOrderData = BasicResponse<AnyResponse>;
pub type CandlesData = BasicResponse<ListResponse<RawCandle>>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    #[serde(deserialize_with = "as_f64")]
    pub total_available_balance: f64,
}
pub type BalanceData = BasicResponse<ListResponse<BalanceResponse>>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub side: Side
}
pub type OrdersData = BasicResponse<ListResponse<OrderResponse>>;

pub struct OrdersCountBySide {
    pub sell: i32,
    pub buy: i32,
}
