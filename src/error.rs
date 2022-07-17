use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostresError {
    #[error("Logging setup error")]
    LoggingSetupError(#[from] tracing::dispatcher::SetGlobalDefaultError),
}
