//! postman
//! Emcapsulates the postman_collection operations

use crate::error::PostresError;
use postman_collection::PostmanCollection;
use tracing::info;

pub(crate) fn load_from_path(path: &str) -> Result<PostmanCollection, PostresError> {
    postman_collection::from_path(path)
        .map_err(|e| PostresError::PostManCollectionParsingError { msg: e.to_string() })
}

pub(crate) fn log_collection_information(collection: &PostmanCollection) {
    match collection {
        PostmanCollection::V1_0_0(spec) => {
            info!("collection {}, version v1.0.0", spec.name);
        }
        PostmanCollection::V2_0_0(spec) => {
            info!("collection {}, version v2.0.0", spec.info.name);
        }
        PostmanCollection::V2_1_0(spec) => {
            info!("collection {}, version v2.1.0", spec.info.name);
        }
    }
}
