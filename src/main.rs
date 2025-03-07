use anyhow::{Error, Result};
use suc::cli::run_cli;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_cli().await?;
    Ok(())
}
