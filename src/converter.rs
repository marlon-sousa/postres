//! The middle: lowers the Postman model into the REST Client model.
//!
//! This is the only module allowed to know both sides, so that a change in either
//! format is absorbed here rather than travelling through the whole program.
