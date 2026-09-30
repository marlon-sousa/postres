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
        let dest_file = dest_file.unwrap_or(source_file.with_extension("http"));
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
    use std::path::Path;

    #[test]
    fn an_explicit_destination_wins() {
        let config = Config::new("users.json".into(), Some("out/requests.http".into()));
        assert_eq!(config.dest_file(), Path::new("out/requests.http"));
    }

    #[test]
    fn only_the_last_extension_is_replaced() {
        let config = Config::new("users.json.json".into(), None);
        assert_eq!(config.dest_file(), Path::new("users.json.http"));
    }

    #[test]
    fn a_directory_name_is_left_alone() {
        let config = Config::new("exports.json/users.json".into(), None);
        assert_eq!(config.dest_file(), Path::new("exports.json/users.http"));
    }

    #[test]
    fn any_extension_is_replaced_not_only_json() {
        let config = Config::new("users.JSON".into(), None);
        assert_eq!(config.dest_file(), Path::new("users.http"));

        let config = Config::new("users.txt".into(), None);
        assert_eq!(config.dest_file(), Path::new("users.http"));
    }

    #[test]
    fn a_file_with_no_extension_gets_one() {
        let config = Config::new("users".into(), None);
        assert_eq!(config.dest_file(), Path::new("users.http"));
    }
}
// ANCHOR_END: tests
