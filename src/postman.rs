//! postman
//! Emcapsulates the postman_collection operations

use std::fs::File;

use crate::error::PostresError;


pub(crate) mod v2_1_0;

use v2_1_0::PostmanCollection;

pub(crate) fn load_from_path(path: &str) -> Result<PostmanCollection, PostresError> {
    let src = File::open(path).map_err(|e| PostresError::SourceFileError { msg: e.to_string() })?;
    let collection: v2_1_0::PostmanCollection = serde_json::from_reader(src)
        .map_err(|e| PostresError::PostManCollectionParsingError { msg: e.to_string() })?;
    Ok(collection)
}
