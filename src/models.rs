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

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CallbackResponse {
    pub api_key: String,
    pub data: CallbackResponseData,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderState {
    Success,
    Canceled,
    CanceledByUser,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CallbackResponseData {
    pub user_id: String,
    pub merchant_secret_key: String,
    pub amount_from_user_in_usdt: Decimal,
    pub amount_from_user_for_merchant: Decimal,
    pub currency_for_users_in_currency: String,
    pub order_state: OrderState,
    #[serde(with = "ts_milliseconds")]
    pub timestamp_created_order: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub timestamp_completed_order: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettingAmount {
    min: Decimal,
    max: Decimal,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSettings {
    amount: SettingAmount,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerchantSettings {
    merchant: WidgetSettings,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub api_key: String,
    pub token: String,
}
