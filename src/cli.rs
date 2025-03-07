use crate::env::Config;
use anyhow::{Error, Result};
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

pub async fn run_cli() -> Result<(), Error> {
    // it loads environment variables
    let config = Config::new()?;

    // it initializes config logging
    setup_tracing()?;

    tracing::info!("hello");

    Ok(())
}

fn setup_tracing() -> Result<(), Error> {
    let log_filter = EnvFilter::from_default_env().add_directive("slack_upload_cli=debug".parse()?);
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
