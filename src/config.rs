/// represents a single configuration for the conversion process
#[derive(Debug)]
pub struct Config {
    /// destination http file
    pub dest_file: String,
    /// source postman file
    pub source_file: String,
}
