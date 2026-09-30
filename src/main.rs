//! The `postres` command line.

use std::path::PathBuf;

use clap::Parser;
use postres::Config;

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
    let config = Config::new(args.postman_file, args.output_file);

    println!(
        "converting {} into {}",
        config.source_file().display(),
        config.dest_file().display()
    );
}
// ANCHOR_END: main
