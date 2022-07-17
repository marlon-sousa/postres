use anyhow::Result;
use clap::Parser;
use tracing::info;

mod config;
mod error;
mod logging;

const APP_NAME: &str = "postres";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short = 'f', long, value_parser)]
    postman_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let subscriber = logging::get_subscriber(APP_NAME, "info", non_blocking_writer);
    logging::init_subscriber(subscriber)?;
    info!("program started");

    Ok(())
}
