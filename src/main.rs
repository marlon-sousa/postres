use anyhow::Result;
use tracing::info;

mod error;
mod logging;

const APP_NAME: &str = "postres";

fn main() -> Result<()> {
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let subscriber = logging::get_subscriber(APP_NAME, "info", non_blocking_writer);
    logging::init_subscriber(subscriber)?;
    info!("program started");
    Ok(())
}
