use errors::ApiError;
use serde::Serialize;

const MOLLIE_API_V2_ENDPOINT: &str = "https://api.mollie.com/v2";

mod endpoints;
pub mod errors;
pub mod resources;

pub struct Client {
    client: reqwest::blocking::Client,
    api_key: String,
    base_url: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::blocking::Client::new();

        Self {
            client,
            api_key,
            base_url: MOLLIE_API_V2_ENDPOINT.to_string(),
        }
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint)
    }

    pub fn get(&self, endpoint: String) -> Result<serde_json::Value, ApiError> {
        let url = self.build_url(&endpoint);

        let response = self
            .client
            .get(url)
            .bearer_auth(self.api_key.clone())
            .send()?
            .error_for_status()?;

        let json = response.json()?;

        Ok(json)
    }

    pub fn patch<T: Serialize>(
        &self,
        endpoint: String,
        parameters: T,
    ) -> Result<serde_json::Value, ApiError> {
        let url = self.build_url(&endpoint);

        let response = self
            .client
            .patch(url)
            .bearer_auth(self.api_key.clone())
            .json(&parameters)
            .send()?
            .error_for_status()?;

        let json = response.json()?;

        Ok(json)
    }
}
