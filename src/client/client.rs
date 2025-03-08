mod get;

use super::{ClientError, Header, RetryPolicy, UrlQuery, format_url};
use crate::traits::Parallelism;
use anyhow::Result;
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Debug;
use tracing::debug;
use url::Url;

#[derive(Clone)]
pub struct Client<T> {
    pub client: ReqwestClient,
    pub base_url: Url,
    pub other_common_config: T,
}

#[trait_variant::make(Send)]
pub trait GetClient {
    async fn get<Response, ResponseErr>(
        &self,
        path: &str,
        header: &Header,
        url_query: &UrlQuery,
    ) -> Result<(Response, Header), ClientError<ResponseErr>>
    where
        Response: DeserializeOwned,
        ResponseErr: DeserializeOwned + Debug + RetryPolicy;
}

#[trait_variant::make(Send)]
pub trait PostJsonClient {
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
pub trait PostMultipartClient {
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
