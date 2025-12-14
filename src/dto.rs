use crate::candle::RawCandle;
use crate::stock::Side;
use serde::Deserialize;
use serde_this_or_that::as_f64;

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
pub struct PaginatedListResponse<T> {
    pub next_page_cursor: String,
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
    pub total_margin_balance: f64,
}
pub type BalanceData = BasicResponse<ListResponse<BalanceResponse>>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub side: Side,
}
pub type OrdersData = BasicResponse<PaginatedListResponse<OrderResponse>>;

pub struct OrdersCountBySide {
    pub sell: i32,
    pub buy: i32,
}
