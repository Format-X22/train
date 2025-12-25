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
pub type LiquidateData = BasicResponse<AnyResponse>;
pub type CancelOrderData = BasicResponse<AnyResponse>;
pub type CandlesData = BasicResponse<ListResponse<RawCandle>>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde(deserialize_with = "as_f64")]
    pub total_margin_balance: f64,
}
pub type BalanceData = BasicResponse<ListResponse<Balance>>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    #[serde(deserialize_with = "as_f64")]
    pub price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub qty: f64,
    pub side: Side,
}
pub type OrdersData = BasicResponse<PaginatedListResponse<Order>>;

impl Clone for Order {
    fn clone(&self) -> Self {
        Self {
            order_id: self.order_id.clone(),
            price: self.price,
            qty: self.qty,
            side: self.side.clone(),
        }
    }
}
