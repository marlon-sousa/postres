use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostresError {
    #[error("Logging setup error")]
    LoggingSetupError(#[from] tracing::dispatcher::SetGlobalDefaultError),
    #[error("Postman collection parser error. Caused by {msg}")]
    PostManCollectionParsingError { msg: String },
    #[error("Required request not find at {path}")]
    PostManRequestNotFound { path: String },
}

impl PostresError {
    pub fn postman_request_not_found(path: impl ToString) -> Self {
        PostresError::PostManRequestNotFound {
            path: path.to_string(),
        }
    }
}
