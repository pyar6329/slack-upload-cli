use anyhow::{Error, Result};
use slack_upload_cli::cli::run_cli;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_cli().await?;
    Ok(())
}
