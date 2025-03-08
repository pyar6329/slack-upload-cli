mod get;

use super::{ClientError, Header, RetryPolicy, UrlQuery, format_url};
use crate::traits::Parallelism;
use anyhow::{Error, Result};
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Debug;
use tokio::time::Duration;
use tracing::debug;
use url::Url;

#[derive(Clone)]
pub struct Client<T> {
    pub(super) client: ReqwestClient,
    pub(super) base_url: Url,
    pub common_config: T,
}

pub trait ClientBuilder<T> {
    fn build(base_url: &Url, base_header: &Header, common_config: &T) -> Result<Client<T>, Error>;
}

#[trait_variant::make(Send)]
pub(super) trait GetClient {
    async fn get_once<Response, ResponseErr>(
        &self,
        path: &str,
        header: &Header,
        url_query: &UrlQuery,
        timeout: &Option<u8>,
    ) -> Result<(Response, Header), ClientError<ResponseErr>>
    where
        Response: DeserializeOwned,
        ResponseErr: DeserializeOwned + Debug + RetryPolicy;
}

#[trait_variant::make(Send)]
pub(super) trait PostJsonClient {
    async fn post_json<Request, Response, ResponseErr>(
        &self,
        path: &str,
        header: &Header,
        request_body: &Request,
    ) -> Result<(Response, Header), ClientError<ResponseErr>>
    where
        Request: Serialize,
        Response: DeserializeOwned,
        ResponseErr: DeserializeOwned + Debug + RetryPolicy;
}

#[trait_variant::make(Send)]
pub(super) trait PostMultipartClient {
    async fn post_multipart<Request, Response, ResponseErr>(
        &self,
        path: &str,
        header: &Header,
        request_body: &Request,
    ) -> Result<(Response, Header), ClientError<ResponseErr>>
    where
        Request: Serialize,
        Response: DeserializeOwned,
        ResponseErr: DeserializeOwned + Debug + RetryPolicy;
}
