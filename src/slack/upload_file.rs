use super::{Client, ErrorInfo};
use anyhow::{Error, Result};
use reqwest::multipart::Form;
use tracing::{debug, error};
use url::Url;

use crate::os::FileInfo;

// ref: https://api.slack.com/methods/files.getUploadURLExternal
pub async fn upload_file(
    client: &Client,
    upload_url: &Url,
    file_info: &FileInfo,
) -> Result<(), Error> {
    let url = Url::parse(upload_url.to_string().as_str())?;
    let form = Form::new()
        //       .text("filetype", "txt")
        //       .text("filename", file_info.file_name.to_owned())
        .file("file", file_info.file_path.to_owned())
        .await
        .inspect_err(|e| error!("failed building form: {:?}", e))?;
    let response = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .inspect_err(|e| error!("failed upload_file sending error: {:?}", e))?;

    let status_code = response.status();
    debug!("upload_file http version: {:?}", response.version());

    let bytes = response.bytes().await?;
    let maybe_error: Result<ErrorInfo, _> = serde_json::from_slice(&bytes);

    match (status_code, maybe_error) {
        (code, _) if code.is_success() => {
            debug!("uploading file was completed!");
            Ok(())
        }
        (_, Ok(error)) => {
            let error_msg = format!(
                "Slack API returns error. error_type: {:?}, error_reasons: {:?}",
                &error.error_type, &error.metadata.reasons
            );
            error!(error_msg);
            Err(Error::msg(error_msg))
        }
        (_, Err(_)) => {
            let err_msg = format!(
                "Slack API error occurred to upload file: {:?}",
                &status_code
            );
            error!(err_msg);
            Err(Error::msg(err_msg))
        }
    }
}
