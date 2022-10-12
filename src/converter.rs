//! converter from postman collection to RestClient format

use crate::{error::PostresError, postman::v2_1_0::PostmanCollection, restclient::HttpRequests};

mod v2_1_0;

/// converts a postman collection to RestClient http format
// This delegates to a specialized module to convert v 2.1.0 collections because if we ever want to add other versions this public api can make the switch without affecting callers
pub fn convert_to_http(collection: &PostmanCollection) -> Result<HttpRequests, PostresError> {
    Ok(v2_1_0::convert_to_http(&collection))
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
