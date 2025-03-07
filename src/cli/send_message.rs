use crate::slack::{Client, send_message};
use anyhow::{Error, Result};
use tracing::info;

pub async fn send_message_to_slack(
    client: &Client,
    slack_channel_id: &str,
    message: &str,
) -> Result<(), Error> {
    let complete_upload = send_message(&client, &slack_channel_id, &message).await?;
    info!("send message: {:?}", &complete_upload);

    Ok(())
}
