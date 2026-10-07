//! The `postres` command line.

use std::io::IsTerminal;
use std::path::PathBuf;

use clap::Parser;
use postres::Config;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};

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

    /// Say what is happening; repeat for more detail
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}
// ANCHOR_END: args

// ANCHOR: main
fn main() {
    let args = Args::parse();
    let _guard = init_logging(args.verbose);

    let config = Config::new(args.postman_file, args.output_file);
    info!(
        source = %config.source_file().display(),
        dest = %config.dest_file().display(),
        "converting"
    );
}
// ANCHOR_END: main

// ANCHOR: logging
/// Sends log lines to standard error through a background thread, so the code
/// doing the work never waits for the terminal.
///
/// Lines are only written for as long as the returned guard is alive: dropping it
/// writes whatever is still queued and stops the thread. Hold it until the end of
/// `main`.
fn init_logging(verbose: u8) -> WorkerGuard {
    let (writer, guard) = NonBlockingBuilder::default()
        .lossy(false)
        .finish(std::io::stderr());

    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_ansi(std::io::stderr().is_terminal())
        .with_max_level(level(verbose))
        .init();

    guard
}
// ANCHOR_END: logging

// ANCHOR: level
/// How much to log for a given number of `-v`: warnings and errors only by
/// default, then info, debug, and everything.
fn level(verbose: u8) -> LevelFilter {
    if verbose == 0 {
        LevelFilter::WARN
    } else if verbose == 1 {
        LevelFilter::INFO
    } else if verbose == 2 {
        LevelFilter::DEBUG
    } else {
        LevelFilter::TRACE
    }
}
// ANCHOR_END: level
