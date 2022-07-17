//! converter from postman collection to RestClient format
use postman_collection::PostmanCollection;

use crate::error::PostresError;
use crate::http_request::HttpRequests;

mod v2_1_0;

/// converts a postman collection to RestClient http format
/// Current supported version is 2.1.0. If other version is supplied, an error is returned
pub fn convert_to_http(collection: &PostmanCollection) -> Result<HttpRequests, PostresError> {
    match collection {
        PostmanCollection::V1_0_0(spec) => Err(
            PostresError::postman_collection_version_not_supported(" 1.0.0"),
        ),
        PostmanCollection::V2_0_0(spec) => Err(
            PostresError::postman_collection_version_not_supported(" 2.0.0"),
        ),
        PostmanCollection::V2_1_0(spec) => Ok(v2_1_0::convert_to_http(&spec)),
    }
}

/// constructs a base name according to the base name and name informed
/// used to form names of requests which are originally inside folders in postman collections.
/// Because RestClient has no concept of folders, the request name of a RestClient request is composed from the name of the folder (s) and the name of the request
fn make_base_name(basename: &str, name: &str) -> String {
    let name = name.replace(" ", "-");
    if basename.is_empty() {
        name
    } else {
        format!("{basename}_{name}")
    }
}
