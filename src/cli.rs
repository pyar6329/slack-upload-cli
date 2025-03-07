mod argument;
mod command;
mod send_message;
mod upload_file;

use crate::env::Config;
use anyhow::{Error, Result};
use tracing::debug;
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

use argument::*;
use command::*;
use send_message::*;
use upload_file::*;

pub async fn run_cli() -> Result<(), Error> {
    // it loads environment variables
    let config = Config::new()?;
    debug!("config: {:?}", &config);

    // it initializes config logging
    setup_tracing(&config)?;

    let arguments = Argument::get();
    Command::from(arguments).run(&config).await?;

    Ok(())
}

fn setup_tracing(config: &Config) -> Result<(), Error> {
    let log_filter = if config.is_debug_build() {
        EnvFilter::from_default_env() // We can use: error!(), warn!(), info!(), debug!()
            .add_directive("slack_upload_cli=debug".parse()?)
    } else {
        EnvFilter::from_default_env() // We can use: error!(), warn!(), info!()
            .add_directive("slack_upload_cli=info".parse()?)
    };

    tracing_subscriber::fmt()
        .json()
        .with_current_span(false)
        .flatten_event(true)
        .with_span_list(true)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(log_filter)
        .init();

    Ok(())
}
