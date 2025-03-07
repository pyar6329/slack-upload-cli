use anyhow::{Error, Result, bail};
use envy::Error as EnvyError;
use serde::Deserialize;
use serde::de::Error as _;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub slack_api_token: String,
    pub slack_channel_id: String,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let envs = envy::from_env::<Self>().map_err(Error::new)?;
        if envs.slack_api_token.is_empty() {
            bail!(EnvyError::custom(
                "cannot set env as empty string: SLACK_API_TOKEN"
            ));
        }

        if envs.slack_channel_id.is_empty() {
            bail!(EnvyError::custom(
                "cannot set env as empty string: SLACK_CHANNEL_ID"
            ));
        }

        Ok(envs)
    }
}
