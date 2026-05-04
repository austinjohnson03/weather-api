use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde::de::DeserializeOwned;
use std::time::Duration;

use crate::error::ClientError;

pub struct Config {
    pub base_url: String,
    pub max_retries: u32,
    pub timeout: u64,
}

pub struct Client {
    client: ClientWithMiddleware,
    pub config: Config,
}

impl Client {
    pub fn new(config: Config) -> Result<Self, ClientError> {
        let inner_builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .map_err(|e| ClientError::BuilderError(e))?;

        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(Duration::from_millis(100), Duration::from_secs(10))
            .build_with_max_retries(config.max_retries);

        Ok(Client {
            client: ClientBuilder::new(inner_builder)
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build(),
            config,
        })
    }

    pub async fn fetch_endpoint<T: DeserializeOwned>(&self, url: &str) -> Result<T, ClientError> {
        let resp = self.client.get(url).send().await?;

        if !resp.status().is_success() {
            return Err(ClientError::HttpError {
                status: resp.status().as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(resp.json::<T>().await?)
    }
}
