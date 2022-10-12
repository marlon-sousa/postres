use derive_builder::{PostBuildError, UninitializedFieldError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostresError {
    #[error("Invalid postman specification: list of items of type {field} is empty")]
    EmptyListOfPostmanItemsError { field: String },
    #[error(transparent)]
    FieldInitializationError(#[from] UninitializedFieldError),
    #[error(transparent)]
    InvalidHttpRequestError(#[from] PostBuildError),
    #[error("Invalid postman form data specification. Caused by {msg}")]
    InvalidPostmanFormDataSpecification { msg: String },
    #[error("Invalid postman form file specification. Caused by {msg}")]
    InvalidPostmanFormFileSpecification { msg: String },
    #[error("Invalid postman graphql specification. Caused by {msg}")]
    InvalidPostmanGraphqlSpecification { msg: String },
    #[error("Invalid postman method : {method}")]
    InvalidPostmanMethod { method: String },
    #[error("Invalid postman header. Caused by {msg}")]
    InvalidPostmanHeader { msg: String },
    #[error("Invalid postman raw specification. Caused by {msg}")]
    InvalidPostmanRawSpecification { msg: String },
    #[error("Logging setup error")]
    LoggingSetupError(#[from] tracing::dispatcher::SetGlobalDefaultError),
    #[error("Postman collection parser error. Caused by {msg}")]
    PostManCollectionParsingError { msg: String },
    #[error("postman collection version not supported error: {version} not supported.")]
    PostmanCollectionVersionNotSupportedError { version: String },
    #[error("Postman file specification not present on request with body of type file")]
    PostmanFileSpecNotPresent,
    #[error("Postman graphql specification not present on request with body of type file")]
    PostmanGraphqlSpecNotPresent,
    #[error("Required request not present at {path}")]
    PostManRequestNotPresent { path: String },
    #[error("postman request method not present")]
    PostmanRequestMethodNotPresent,
    #[error("postman request url not present")]
    PostmanRequestUrlNotPresent,
    #[error("Postman url encoded specification not present on request with body of type file")]
    PostmanUrlEncodedSpecNotPresent,
    #[error("Could  not load source file. Caused by {msg}")]
    SourceFileError { msg: String },
}

impl PostresError {
    pub fn invalid_postman_method(method: &str) -> Self {
        Self::InvalidPostmanMethod {
            method: method.to_string(),
        }
    }

    pub fn invalid_postman_form_data_specification(msg: impl ToString) -> Self {
        Self::InvalidPostmanFormDataSpecification {
            msg: msg.to_string(),
        }
    }

    pub fn invalid_postman_form_for_file_specification(msg: impl ToString) -> Self {
        Self::InvalidPostmanFormFileSpecification {
            msg: msg.to_string(),
        }
    }
    pub fn invalid_postman_header(msg: impl ToString) -> Self {
        Self::InvalidPostmanHeader {
            msg: msg.to_string(),
        }
    }

    pub fn postman_collection_version_not_supported(version: impl ToString) -> Self {
        PostresError::PostmanCollectionVersionNotSupportedError {
            version: version.to_string(),
        }
    }

    pub fn postman_request_not_present(path: impl ToString) -> Self {
        PostresError::PostManRequestNotPresent {
            path: path.to_string(),
        }
    }
}
