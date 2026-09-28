//! Convert a Postman collection into `.http` files for the VS Code REST Client
//! extension.
//!
//! The conversion is a small compiler. A collection is read and parsed into a
//! model of what Postman describes, that model is lowered into a second one
//! describing what REST Client understands, and the second model is written out
//! as a file. Anything that goes wrong on the way is reported by name.
//!
//! Nothing here is public yet. Each stage becomes visible when there is something
//! in it worth calling.

// ANCHOR: modules
// Reads a Postman collection and parses it into the source model.
mod postman;

// Lowers the source model into the target model. The only stage that knows both.
mod converter;

// The target model, and the writer that turns it into a `.http` file.
mod restclient;

// Everything that can go wrong, with a name.
mod error;
// ANCHOR_END: modules
