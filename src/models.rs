use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeTokenRequest {
    pub api_key: String,
    pub secret_key: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeTokenResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CallbackResponse {
    pub api_key: String,
    pub data: CallbackResponseData,
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CallbackResponseData {
    pub user_id: String,
    pub merchant_secret_key: String,
    pub amount_from_user_in_usdt: Decimal,
    pub amount_from_user_for_merchant: Decimal,
    #[serde(with = "ts_milliseconds")]
    pub timestamp_created_order: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub timestamp_completed_order: DateTime<Utc>,
}
