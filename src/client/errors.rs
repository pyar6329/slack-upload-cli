use super::Header;
use reqwest::{Error as ReqwestError, StatusCode};
use serde_json::Error as JsonError;
use std::fmt::Debug;
use strum::EnumIs;
use thiserror::Error as ThisError;
use tracing::{error, warn};

#[derive(Debug, Clone, PartialEq, Eq, EnumIs, ThisError)]
pub enum ClientError<T> {
    #[error("reqwest client building is failed: {0}")]
    BuildFailed(String),
    #[error("reqwest serde_json parse error: {0}")]
    ParseJsonError(String),
    #[error("reqwest .bytes() error: {0}")]
    ParseBytesError(String),
    #[error("reqwest send error: {0}")]
    SendError(String),
    #[error(
        "reqwest send was succeed, however response returns error: {0:?}, status_code: {1}, header: {2:?}"
    )]
    ResponseError(T, StatusCode, Header),
    #[error(
        "reqwest was retried many times, however response returns error, so retrying was cancelled: {0:?}, status_code: {1}, header: {2:?}"
    )]
    ReachedRetryNum(T, StatusCode, Header),
    #[error("reqwest was send tokio::spawn, however it was failed to receive from parallel task")]
    CannotReceiveFromParallel,
    #[error("reqwest returns unknown error occurred")]
    Unknown,
}

impl<T> ClientError<T>
where
    T: Debug,
{
    pub fn build_failed(error: ReqwestError) -> Self {
        let err = Self::BuildFailed(error.to_string());
        error!("{}", err);
        err
    }

    pub fn parse_json_error(error: JsonError) -> Self {
        let err = Self::ParseJsonError(error.to_string());
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

    pub fn response_error(error: T, status_code: &StatusCode, header: &Header) -> Self {
        let err = Self::ResponseError(error, *status_code, header.to_owned());
        error!("{}", err);
        err
    }

    pub fn reached_retry_num(error: T, status_code: &StatusCode, header: &Header) -> Self {
        let err = Self::ResponseError(error, *status_code, header.to_owned());
        error!("{}", err);
        err
    }

    pub fn cannot_receive_from_parallel() -> Self {
        let err = Self::CannotReceiveFromParallel;
        error!("{}", err);
        err
    }

    pub fn unknown() -> Self {
        let err = Self::Unknown;
        error!("{}", err);
        err
    }
}
