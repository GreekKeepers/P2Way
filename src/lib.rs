pub mod models;
use std::sync::Arc;

pub use models::*;
pub mod errors;

use reqwest::{Client, Method};

pub static SANDBOX_URL: &str = "https://dev.p2way.fyi";
pub static PROD_URL: &str = "https://p2way.fyi";

#[derive(Clone)]
pub struct P2Way {
    api_key: String,
    secret_key: String,
    base_url: &'static str,
    client: Arc<Client>,
    baked_ott_request: String,
}

impl P2Way {
    pub fn new(api_key: String, secret_key: String, base_url: &'static str) -> Self {
        Self {
            api_key: api_key.clone(),
            secret_key: secret_key.clone(),
            base_url,
            client: Arc::new(Client::new()),
            baked_ott_request: serde_json::to_string(&OneTimeTokenRequest {
                api_key,
                secret_key,
            })
            .unwrap(),
        }
    }

    pub async fn one_time_token_generation(&self) -> Result<OneTimeTokenResponse, errors::Error> {
        let url = format!("{}{}", self.base_url, "/merchant/generateOneTimeToken");
        let res = self
            .client
            .post(url)
            .body(self.baked_ott_request.clone())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(errors::Error::RequestError)?
            .text()
            .await
            .map_err(errors::Error::RequestError)?;

        Ok(serde_json::from_str(&res).map_err(|e| errors::Error::SerdeError(e, res))?)
    }
}
