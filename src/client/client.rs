use super::{ClientError, Header, UrlQuery, format_url};
use anyhow::{Error, Result};
use reqwest::{Client as ReqwestClient, StatusCode};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tracing::debug;
use url::Url;

#[derive(Clone)]
pub struct Client<T: Clone> {
    pub client: ReqwestClient,
    pub base_url: Url,
    pub other_common_config: T,
}

impl<T: Clone> Client<T> {
    async fn get<Response, ResponseErr>(
        &self,
        path: &str,
        header: &Header,
        url_query: &UrlQuery,
    ) -> Result<(Response, Header), ClientError<ResponseErr>>
    where
        Response: DeserializeOwned,
        ResponseErr: DeserializeOwned + Debug,
    {
        let url = format_url(&self.base_url, path, url_query);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(ClientError::send_error)?;

        debug!("get request Response: {:?}", &response);

        let status_code = response.status();
        let response_header: Header = response.headers().into();
        let response_body_bytes = response
            .bytes()
            .await
            .map_err(ClientError::parse_bytes_error)?;

        let maybe_response: Result<Response, ClientError<ResponseErr>> =
            serde_json::from_slice(&response_body_bytes).map_err(ClientError::parse_json_error);

        let maybe_response_err: Result<ResponseErr, ClientError<ResponseErr>> =
            serde_json::from_slice(&response_body_bytes).map_err(ClientError::parse_json_error);

        match (status_code, maybe_response, maybe_response_err) {
            (_, Ok(response), _) => Ok((response, response_header)),
            (_, Err(_), Ok(response_err)) => Err(ClientError::response_error(
                response_err,
                &status_code,
                &response_header,
            )),
            (status, Err(e), Err(_))
                if status.is_success() || status.is_redirection() | status.is_informational() =>
            {
                Err(e)
            }
            (status, Err(_), Err(e)) if status.is_client_error() || status.is_server_error() => {
                Err(e)
            }
            (_, Err(_), Err(e)) => Err(e),
        }
    }
}
