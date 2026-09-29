//! The `postres` command line.

use std::path::PathBuf;

use clap::Parser;

// ANCHOR: args
// The command line, as clap parses it. `about` is the description in Cargo.toml,
// and each field's doc comment is its line in --help.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// The Postman collection to convert, exported as v2.1 JSON
    #[arg(short = 'f', long)]
    postman_file: PathBuf,

    /// Where to write the .http file [default: the collection's path, ending in .http]
    #[arg(short, long)]
    output_file: Option<PathBuf>,
}
// ANCHOR_END: args

// ANCHOR: main
fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
// ANCHOR_END: main
