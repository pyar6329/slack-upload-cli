use super::{Client, ClientBuilder, ClientError, Header};
use crate::traits::Parallelism;
use anyhow::{Error, Result};
use reqwest::Client as ReqwestClient;
use std::fmt::Debug;
use tokio::time::Duration;
use url::Url;

const DEFAULT_REQUEST_TIMEOUT: u64 = 10;
const DEFAULT_CONNECTION_TIMEOUT: u64 = 3;

const DEFAULT_KEEP_ALIVE_INTERVAL: u64 = 20;
const DEFAULT_KEEP_ALIVE_TIMEOUT: u64 = 10;

const DEFAULT_CONNECTION_POOL_MAX_SIZE: usize = 32;
const DEFAULT_CONNECTION_POOL_IDLE_TIMEOUT: u64 = 90;

impl<T> ClientBuilder<T> for Client<T>
where
    T: Clone + Debug + Parallelism,
{
    fn build(base_url: &Url, base_header: &Header, common_config: &T) -> Result<Self, Error> {
        let client = ReqwestClient::builder()
            .default_headers(base_header.to_owned().into())
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
            .map_err(ClientError::<T>::build_failed)
            .map_err(Error::new)?;

        let client_dataset = Self {
            client,
            base_url: base_url.to_owned(),
            common_config: common_config.to_owned(),
        };
        Ok(client_dataset)
    }
}
