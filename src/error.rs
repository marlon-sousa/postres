use thiserror::Error;

#[derive(Error, Debug)]
enum PostresError {
    #[error("Logging setup error")]
    LoggingSetupError(#[from] tracing::dispatcher::SetGlobalDefaultError),
}
