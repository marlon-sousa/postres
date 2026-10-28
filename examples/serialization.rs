//! Serialization, read from the receiving end: the sample collection read first as
//! plain JSON, then into types of our own, and then the same types read from TOML.
//!
//! Run it with `cargo run --example serialization`.

// The model's fields are only ever printed through `Debug`, which the compiler does
// not count as reading them.
#![allow(dead_code)]

use std::fs;

use serde::Deserialize;

// ANCHOR: model
#[derive(Debug, Deserialize)]
struct Collection {
    info: Info,
    #[serde(rename = "item")]
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Item {
    name: Option<String>,
    #[serde(rename = "item")]
    items: Option<Vec<Item>>,
    request: Option<Request>,
}

#[derive(Debug, Deserialize)]
struct Request {
    method: Option<String>,
}
// ANCHOR_END: model

// ANCHOR: toml
const AS_TOML: &str = r#"
[info]
name = "Users API"

[[item]]
name = "Health check"
request = { method = "GET" }
"#;
// ANCHOR_END: toml

fn main() {
    let text = fs::read_to_string("samples/users.json").expect("run it from the repository root");

    // ANCHOR: value
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    println!("{}", value["info"]["name"]);
    println!("{}", value["info"]["nmae"]);
    println!("{}", value["item"][0]["item"][1]["request"]["method"]);
    // ANCHOR_END: value

    println!();

    // ANCHOR: types
    let collection: Collection = serde_json::from_str(&text).unwrap();
    println!("{collection:#?}");
    // ANCHOR_END: types

    println!();

    // ANCHOR: formats
    let from_toml: Collection = toml::from_str(AS_TOML).unwrap();
    println!("{from_toml:?}");
    // ANCHOR_END: formats
}
