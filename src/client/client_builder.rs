use crate::env::Config;
use anyhow::{Error, Result};
use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap},
};
use std::collections::HashMap;
use tokio::time::Duration;

const DEFAULT_REQUEST_TIMEOUT: u64 = 10;
const DEFAULT_CONNECTION_TIMEOUT: u64 = 3;

const DEFAULT_KEEP_ALIVE_INTERVAL: u64 = 20;
const DEFAULT_KEEP_ALIVE_TIMEOUT: u64 = 10;

const DEFAULT_CONNECTION_POOL_MAX_SIZE: usize = 32;
const DEFAULT_CONNECTION_POOL_IDLE_TIMEOUT: u64 = 90;

type Headers = HashMap<String, String>;

pub struct ClientBuilder {
    pub headers: Headers,
    pub slack_channel_id: String,
}

impl ClientBuilder {
    pub fn build(&self) -> Result<Client, Error> {
        Client::builder()
            .default_headers(self.get_headers(true))
            .use_rustls_tls()
            // .http2_prior_knowledge() // force HTTP/2
            .http2_adaptive_window(true)
            // .http2_max_frame_size(1 << 24 - 1) // allow body to 16MB frame size
            .timeout(Duration::from_secs(DEFAULT_REQUEST_TIMEOUT))
            .connect_timeout(Duration::from_secs(DEFAULT_CONNECTION_TIMEOUT))
            .pool_idle_timeout(Duration::from_secs(DEFAULT_CONNECTION_POOL_IDLE_TIMEOUT))
            .pool_max_idle_per_host(DEFAULT_CONNECTION_POOL_MAX_SIZE)
            .http2_keep_alive_interval(Duration::from_secs(DEFAULT_KEEP_ALIVE_INTERVAL))
            .http2_keep_alive_timeout(Duration::from_secs(DEFAULT_KEEP_ALIVE_TIMEOUT))
            .http2_keep_alive_while_idle(true)
            .build()
            .map_err(Error::new)
    }

    pub fn get_channel_id(&self) -> String {
        self.slack_channel_id.to_owned()
    }

    fn get_headers(&self, use_content_type: bool) -> HeaderMap {
        let mut headers = HeaderMap::try_from(&self.headers).unwrap_or_default();
        if use_content_type {
            headers.insert(
                CONTENT_TYPE,
                "application/json; charset=utf-8".parse().unwrap(),
            );
        }
        headers
    }

    fn build_header_hashes(slack_api_token: &str) -> Headers {
        let authz_token = format!("Bearer {}", slack_api_token);
        HashMap::from([(AUTHORIZATION.to_string(), authz_token)])
    }
}

impl From<Config> for ClientBuilder {
    fn from(config: Config) -> Self {
        Self::from(&config)
    }
}

impl From<&Config> for ClientBuilder {
    fn from(config: &Config) -> Self {
        let headers = Self::build_header_hashes(&config.slack_api_token);
        Self {
            headers,
            slack_channel_id: config.slack_channel_id.to_owned(),
        }
    }
}
