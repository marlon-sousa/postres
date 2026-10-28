// ANCHOR: overview
//! The source side: a Postman collection, read from disk and parsed into types
//! that follow the Postman v2.1 schema.
//!
//! Nothing in this module knows that REST Client exists.
// ANCHOR_END: overview

use postman_collection::PostmanCollection;

// ANCHOR: collection
/// A Postman collection in the v2.1 format, as the `postman_collection` crate
/// models it: every field the schema declares.
pub use postman_collection::v2_1_0::Spec as Collection;
// ANCHOR_END: collection

// ANCHOR: read
/// Reads a collection from the text of an export.
///
/// # Panics
///
/// When the text is not a Postman collection, or is one in a version other than
/// v2.1. Saying so properly is still to come.
pub fn read_collection(text: &str) -> Collection {
    let collection = postman_collection::from_str(text).expect("not a Postman collection");
    match collection {
        PostmanCollection::V2_1_0(spec) => spec,
        other => panic!(
            "postres converts v2.1 collections, and this one is {:?}",
            other.version()
        ),
    }
}
// ANCHOR_END: read

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sample_collection_is_read() {
        let collection = read_collection(include_str!("../samples/users.json"));
        assert_eq!(collection.info.name, "Users API");
        assert_eq!(collection.item.len(), 3);
    }

    #[test]
    fn a_request_written_as_a_bare_url_is_read() {
        let collection = read_collection(
            r#"{
                "info": {
                    "name": "Bare",
                    "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
                },
                "item": [{ "name": "Ping", "request": "https://example.com/ping" }]
            }"#,
        );
        assert_eq!(collection.item.len(), 1);
    }
}
// ANCHOR_END: tests
