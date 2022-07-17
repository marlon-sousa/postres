use tracing::info;

mod logging;

const APP_NAME: &str = "postres";

fn main() {
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let subscriber = logging::get_subscriber(APP_NAME, "info", non_blocking_writer);
    logging::init_subscriber(subscriber);
    info!("program started");
}
