use crate::os::FileInfo;
use crate::slack::{Client, complete_upload, get_upload_url, upload_file};
use anyhow::{Error, Result};
use tracing::info;

pub async fn upload_file_to_slack(
    client: &Client,
    slack_channel_id: &str,
    file_path: &str,
) -> Result<(), Error> {
    let file_info = FileInfo::new(file_path)?;

    let upload_info = get_upload_url(client, &file_info).await?;
    info!("get Upload URL: {:?}", &upload_info);

    upload_file(client, &upload_info.url, &file_info).await?;
    info!("uploaded file: {:?}", &());

    let complete_upload =
        complete_upload(client, slack_channel_id, &file_info, &upload_info).await?;
    info!("uploaded file: {:?}", &complete_upload);

    Ok(())
}
