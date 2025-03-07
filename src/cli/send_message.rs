use crate::os::FileInfo;
use crate::slack::{Client, complete_upload, get_upload_url, upload_file};
use anyhow::{Error, Result};
use tracing::info;

pub async fn send_message_to_slack(
    client: &Client,
    slack_channel_id: &str,
    file_path: &str,
) -> Result<(), Error> {
    let file_info = FileInfo::new(file_path)?;

    let upload_info = get_upload_url(&client, &file_info).await?;
    info!("get Upload URL: {:?}", &upload_info);

    let upload_result = upload_file(&client, &upload_info.url, &file_info).await?;
    info!("uploaded file: {:?}", &upload_result);

    let complete_upload =
        complete_upload(&client, &file_info, &upload_info, &slack_channel_id).await?;
    info!("uploaded file: {:?}", &complete_upload);

    Ok(())
}
