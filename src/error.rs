use derive_builder::UninitializedFieldError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostresError {
    #[error(transparent)]
    FieldInitializationError(#[from] UninitializedFieldError),
    #[error("Logging setup error")]
    LoggingSetupError(#[from] tracing::dispatcher::SetGlobalDefaultError),
    #[error("Postman collection parser error. Caused by {msg}")]
    PostManCollectionParsingError { msg: String },
    #[error("postman collection version not supported error: {version} not supported.")]
    PostmanCollectionVersionNotSupportedError { version: String },
    #[error("Required request not find at {path}")]
    PostManRequestNotFound { path: String },
}

impl PostresError {
    pub fn postman_collection_version_not_supported(version: impl ToString) -> Self {
        PostresError::PostmanCollectionVersionNotSupportedError {
            version: version.to_string(),
        }
    }

    pub fn postman_request_not_found(path: impl ToString) -> Self {
        PostresError::PostManRequestNotFound {
            path: path.to_string(),
        }
    }
}
