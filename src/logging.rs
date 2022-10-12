use tracing::subscriber::{set_global_default, Subscriber};
use tracing_appender::non_blocking::NonBlocking;
use tracing_log::LogTracer;
use tracing_subscriber::EnvFilter;

pub fn get_subscriber(
    name: &str,
    env_filter: &str,
    non_blocking_writer: NonBlocking,
) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    tracing_subscriber::fmt()
        .with_writer(non_blocking_writer)
        .with_env_filter(env_filter)
        .finish()
}

pub fn init_subscriber(
    subscriber: impl Subscriber + Send + Sync,
) -> Result<(), tracing::dispatcher::SetGlobalDefaultError> {
    LogTracer::init().expect("Failed to init logging sub systems");
    set_global_default(subscriber)
}
