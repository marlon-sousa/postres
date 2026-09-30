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
    /// When no destination is given, it is the collection's own path with its
    /// extension replaced by `.http`, next to the collection:
    ///
    /// ```
    /// use postres::Config;
    /// use std::path::{Path, PathBuf};
    ///
    /// let config = Config::new(PathBuf::from("users.json"), None);
    /// assert_eq!(config.dest_file(), Path::new("users.http"));
    /// ```
    pub fn new(source_file: PathBuf, dest_file: Option<PathBuf>) -> Self {
        let dest_file = dest_file.unwrap_or_else(|| source_file.with_extension("http"));
        Self {
            source_file,
            dest_file,
        }
    }
}
// ANCHOR_END: new

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::path::Path;

    #[test]
    fn an_explicit_destination_wins() {
        let config = Config::new("users.json".into(), Some("out/requests.http".into()));
        assert_eq!(config.dest_file(), Path::new("out/requests.http"));
    }

    // ANCHOR: table
    #[rstest]
    #[case::only_the_last_extension_is_replaced("users.json.json", "users.json.http")]
    #[case::a_directory_name_is_left_alone("exports.json/users.json", "exports.json/users.http")]
    #[case::an_uppercase_extension_is_replaced("users.JSON", "users.http")]
    #[case::any_extension_is_replaced("users.txt", "users.http")]
    #[case::a_file_with_no_extension_gets_one("users", "users.http")]
    fn the_default_name(#[case] source: PathBuf, #[case] expected: PathBuf) {
        let config = Config::new(source, None);
        assert_eq!(config.dest_file(), expected.as_path());
    }
    // ANCHOR_END: table
}
// ANCHOR_END: tests
