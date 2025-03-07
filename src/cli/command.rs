use crate::env::Config;
use crate::slack::ClientBuilder;
use anyhow::{Error, Result};
use strum::EnumIs;
use tracing::debug;

use super::{Argument, send_message_to_slack, upload_file_to_slack};

type FilePathString = String;
#[derive(Clone, Debug, PartialEq, Eq, EnumIs, Default)]
pub(super) enum Command {
    #[default]
    DoNothing,
    Upload(FilePathString),
    SendMessage(String),
}

impl From<Argument> for Command {
    fn from(arg: Argument) -> Self {
        use Command::*;

        if let Some(path) = arg.upload {
            return Upload(path);
        }

        if let Some(path) = arg.send {
            return SendMessage(path);
        }

        Command::DoNothing
    }
}

impl Command {
    pub async fn run(&self, config: &Config) -> Result<(), Error> {
        if self.is_do_nothing() {
            return Ok(());
        }

        let client_builder = ClientBuilder::from(config);
        let slack_channel_id = client_builder.get_channel_id();
        let client = client_builder.build()?;
        debug!("create client");

        match self {
            Command::DoNothing => Ok(()),
            Command::Upload(file_path) => {
                upload_file_to_slack(&client, &slack_channel_id, file_path).await
            }
            Command::SendMessage(message) => {
                send_message_to_slack(&client, &slack_channel_id, message).await
            }
        }
    }
}
