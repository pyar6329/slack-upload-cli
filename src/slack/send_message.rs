use super::{Client, ErrorInfo};
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tracing::{debug, error};
use url::Url;

const API_URL: &str = "https://slack.com/api/chat.postMessage";

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct SentMessage {
    #[serde(rename = "channel")]
    pub channel_id: String,
    #[serde(rename = "ts")]
    pub timestamp: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
struct RequestBody {
    #[serde(rename = "channel")]
    pub channel_id: String,
    #[serde(rename = "text")]
    pub message: String,
}

// ref: https://api.slack.com/methods/chat.postMessage
pub async fn send_message(
    client: &Client,
    slack_channel_id: &str,
    message: &str,
) -> Result<SentMessage, Error> {
    let url = Url::parse(API_URL)?;
    let request_body = RequestBody {
        channel_id: slack_channel_id.to_string(),
        message: message.to_string(),
    };

    let response = client
        .post(url)
        .json(&request_body)
        .send()
        .await
        .inspect_err(|e| error!("failed send_message sending error: {:?}", e))?;

    debug!("send_message http version: {:?}", response.version());

    let bytes = response.bytes().await?;
    let maybe_succeed_data: Result<SentMessage, _> = serde_json::from_slice(&bytes);
    let maybe_error: Result<ErrorInfo, _> = serde_json::from_slice(&bytes);

    match (maybe_succeed_data, maybe_error) {
        (Ok(data), _) => {
            let json_value: JsonValue = serde_json::from_slice(&bytes).unwrap_or_default();
            debug!("send_message body: {:?}", &json_value);
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
