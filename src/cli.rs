use crate::env::Config;
use crate::os::FileInfo;
use crate::slack::{ClientBuilder, complete_upload, get_upload_url, upload_file};
use anyhow::{Error, Result};
use tracing::{debug, info};
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

pub async fn run_cli() -> Result<(), Error> {
    // it loads environment variables
    let config = Config::new()?;
    debug!("config: {:?}", &config);

    // it initializes config logging
    setup_tracing(&config)?;

    let client_builder = ClientBuilder::from(&config);
    let client = client_builder.build()?;
    debug!("create client");

    let file_name = "foo.txt";
    let file_info = FileInfo::new(file_name)?;

    let upload_info = get_upload_url(&client, &file_info).await?;
    info!("get Upload URL: {:?}", &upload_info);

    let upload_result = upload_file(&client, &upload_info.url, &file_info).await?;
    info!("uploaded file: {:?}", &upload_result);

    let slack_channel_id = client_builder.get_channel_id();

    let complete_upload =
        complete_upload(&client, &file_info, &upload_info, &slack_channel_id).await?;
    info!("uploaded file: {:?}", &complete_upload);

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
        // .json()
        // .with_current_span(false)
        // .flatten_event(true)
        // .with_span_list(true)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(log_filter)
        .init();

    Ok(())
}
