
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Error,
};
use serde_json::Value;
use crate::http_client::HttpClient;

pub struct GoPlusService {
    http_client: HttpClient,
    api_base_url: String,
}

impl GoPlusService {
    pub fn new(http_client: HttpClient) -> Self {
        GoPlusService {
            http_client,
            api_base_url: "https://api.gopluslabs.io/api/v1/".to_string(),
        }
    }

    pub async fn get_token_security(
        &self,
        token_address: &str,
        chain_id: u32,
    ) -> Result<Value, Error> {
        self.http_client
            .retry(|| async {
                let url = format!(
                    "{}token_security/{}?contract_addresses={}",
                    self.api_base_url, chain_id, token_address
                );

                let mut headers = HeaderMap::new();
                headers.insert("accept", HeaderValue::from_static("application/json"));

                let response = self
                    .http_client
                    .get_client()
                    .get(&url)
                    .headers(headers)
                    .send()
                    .await?;

                response.json().await.map_err(Error::from)
            })
            .await
    }
}
