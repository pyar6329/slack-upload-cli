use super::{Client, ErrorInfo};
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tracing::{debug, error};
use url::Url;

use super::UploadInfo;
use crate::os::FileInfo;

const API_URL: &str = "https://slack.com/api/files.completeUploadExternal";

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct CompletedUploadInfo {
    #[serde(rename = "files")]
    pub files: Vec<UploadedFileInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UploadedFileInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct RequestBody {
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    #[serde(rename = "files")]
    pub files: Vec<UploadedFileInfo>,
}

// ref: https://api.slack.com/methods/files.completeUploadExternal
pub async fn complete_upload(
    client: &Client,
    file_info: &FileInfo,
    upload_info: &UploadInfo,
    slack_channel_id: &str,
) -> Result<CompletedUploadInfo, Error> {
    let url = Url::parse(API_URL)?;
    let uploaded_file = UploadedFileInfo {
        id: upload_info.id.to_owned(),
        title: file_info.file_name.to_owned(),
    };
    let request_body = RequestBody {
        channel_id: slack_channel_id.to_string(),
        files: vec![uploaded_file],
    };

    let response = client
        .post(url)
        .json(&request_body)
        .send()
        .await
        .inspect_err(|e| error!("failed complete_upload sending error: {:?}", e))?;

    debug!("complete_upload http version: {:?}", response.version());

    let bytes = response.bytes().await?;
    let maybe_succeed_data: Result<CompletedUploadInfo, _> = serde_json::from_slice(&bytes);
    let maybe_error: Result<ErrorInfo, _> = serde_json::from_slice(&bytes);

    match (maybe_succeed_data, maybe_error) {
        (Ok(data), _) => {
            let json_value: JsonValue = serde_json::from_slice(&bytes).unwrap_or_default();
            debug!("complete upload body: {:?}", &json_value);
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
