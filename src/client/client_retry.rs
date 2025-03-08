mod get;

use super::{Client, ClientError, GetClient, Header, UrlQuery};
use crate::traits::Parallelism;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tokio::sync::oneshot;
use tokio::time::{Duration, sleep};

#[trait_variant::make(Send)]
pub trait GetWithRetryClient {
    async fn get_with_retry<Response, ResponseErr>(
        &self,
        path: &str,
        header: &Header,
        url_query: &UrlQuery,
        retry_num: &u8,
        timeout_per_once: &Option<u8>,
    ) -> Result<(Response, Header), ClientError<ResponseErr>>
    where
        Response: DeserializeOwned + Parallelism,
        ResponseErr: DeserializeOwned + Debug + RetryPolicy + Parallelism;
}

pub trait RetryPolicy {
    fn should_retry(&self, status_code: &StatusCode, header: &Header) -> bool;
}
