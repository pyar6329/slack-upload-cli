use super::{Client, ErrorInfo};
use anyhow::{Error, Result};
use serde::Deserialize;
use serde_json::Value as JsonValue;
use tracing::{debug, error};
use url::Url;

use crate::os::FileInfo;

const API_URL: &str = "https://slack.com/api/files.getUploadURLExternal";

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct UploadInfo {
    #[serde(rename = "upload_url")]
    pub url: Url,
    #[serde(rename = "file_id")]
    pub id: String,
}

// ref: https://api.slack.com/methods/files.getUploadURLExternal
pub async fn get_upload_url(client: &Client, file_info: &FileInfo) -> Result<UploadInfo, Error> {
    let url = Url::parse_with_params(
        API_URL,
        &[
            ("filename", file_info.file_name.to_owned()),
            ("length", file_info.file_size.to_string()),
        ],
    )?;
    let response = client
        .get(url)
        .send()
        .await
        .inspect_err(|e| error!("failed get_upload_url sending error: {:?}", e))?;

    debug!("get_upload_url http version: {:?}", response.version());

    let bytes = response.bytes().await?;
    let maybe_succeed_data: Result<UploadInfo, _> = serde_json::from_slice(&bytes);
    let maybe_error: Result<ErrorInfo, _> = serde_json::from_slice(&bytes);

    match (maybe_succeed_data, maybe_error) {
        (Ok(data), _) => {
            let json_value: JsonValue = serde_json::from_slice(&bytes).unwrap_or_default();
            debug!("get upload url: {:?}", &json_value);
            Ok(data)
        }
        (_, Ok(error)) => {
            let error_msg = format!(
                "Slack API returns error. error_type: {:?}, error_reasons: {:?}",
                &error.error_type, &error.metadata.reasons
            );
            error!(error_msg);
            Err(Error::msg(error_msg))
        }
        (Err(err1), Err(err2)) => {
            let json_value: JsonValue = serde_json::from_slice(&bytes).unwrap_or_default();
            let err_msg = format!(
                "Slack API parse JSON error: {:?}, {:?}, json value: {:?}",
                err1,
                err2,
                &json_value.to_string()
            );
            error!(err_msg);
            Err(Error::msg(err_msg))
        }
    }
}
