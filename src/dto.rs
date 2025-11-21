use serde::Deserialize;
use crate::candle::RawCandle;

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
