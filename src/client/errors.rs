use super::Header;
use reqwest::{Error as ReqwestError, StatusCode};
use serde_json::Error as JsonError;
use strum::EnumIs;
use thiserror::Error as ThisError;
use tracing::{error, warn};

type RetriedNum = u8;

#[derive(Debug, Clone, PartialEq, Eq, EnumIs, ThisError)]
pub enum ClientError {
    #[error("reqwest serde_json parse error: {0}")]
    ParseJsonError(String),
    #[error("reqwest .bytes() error: {0}")]
    ParseBytesError(String),
    #[error("reqwest send error: {0}")]
    SendError(String),
    #[error("reqwest send error but it can retry calling with {1} times: {0}")]
    RetryableError(String, RetriedNum, StatusCode, Header),
    #[error("reqwest send and retried, however it reached max retry count: {0}")]
    MaxRetryError(String, StatusCode, Header),
}

impl From<JsonError> for ClientError {
    fn from(error: JsonError) -> Self {
        Self::ParseJsonError(error.to_string())
    }
}

impl ClientError {
    pub fn parse_json_error(error: JsonError) -> Self {
        let err = Self::from(error);
        error!("{}", err);
        err
    }

    pub fn parse_bytes_error(error: ReqwestError) -> Self {
        let err = Self::ParseBytesError(error.to_string());
        error!("{}", err);
        err
    }

    pub fn send_error(error: ReqwestError) -> Self {
        let err = Self::SendError(error.to_string());
        warn!("{}", err);
        err
    }

    pub fn retryable_error(
        error: ReqwestError,
        retried_num: &RetriedNum,
        status_code: &StatusCode,
        header: &Header,
    ) -> Self {
        let err = Self::RetryableError(
            error.to_string(),
            *retried_num,
            *status_code,
            header.to_owned(),
        );
        warn!("{}", err);
        err
    }

    pub fn max_retry_error(error: ReqwestError, status_code: &StatusCode, header: &Header) -> Self {
        let err = Self::MaxRetryError(error.to_string(), *status_code, header.to_owned());
        error!("{}", err);
        err
    }
}
