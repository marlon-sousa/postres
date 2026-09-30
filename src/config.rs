//! What one run of the converter has been asked to do.

use std::path::PathBuf;

use getset::Getters;

// ANCHOR: config
/// What one conversion has been asked to do: which collection to read, and where
/// to write the result.
///
/// A `Config` is settled the moment it is built. Its fields are private, and it
/// can only be read, never changed.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Config {
    /// The Postman collection to read.
    source_file: PathBuf,
    /// The `.http` file to write.
    dest_file: PathBuf,
}
// ANCHOR_END: config

// ANCHOR: new
impl Config {
    /// Builds a configuration from the paths the user gave.
    ///
    /// When no destination is given, it is the collection's own path with `.json`
    /// replaced by `.http`.
    pub fn new(source_file: PathBuf, dest_file: Option<PathBuf>) -> Self {
        let dest_file = dest_file.unwrap_or(PathBuf::from(
            source_file.to_string_lossy().replace(".json", ".http"),
        ));
        Self {
            source_file,
            dest_file,
        }
    }
}
// ANCHOR_END: new
