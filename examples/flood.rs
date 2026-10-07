//! A flood of log lines: eight threads, each logging a hundred thousand lines to a
//! file, and how many of them actually reach it.
//!
//! Run it with one of three ways of writing the log:
//!
//! ```text
//! cargo run --release --example flood -- sync
//! cargo run --release --example flood -- lossy
//! cargo run --release --example flood -- blocking
//! ```

use std::fs::File;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

use tracing::info;
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};

const THREADS: u64 = 8;
const LINES: u64 = 100_000;

// ANCHOR: start_logging
/// Sends the log to `file` in one of three ways: every thread writing to the file
/// itself, behind a lock (`sync`), or through a queue that drops lines when it is
/// full (`lossy`) or makes the caller wait instead (otherwise).
fn start_logging(sync: bool, lossy: bool, file: File) -> Option<WorkerGuard> {
    if sync {
        tracing_subscriber::fmt()
            .with_ansi(false)
            .with_writer(Mutex::new(file))
            .init();
        None
    } else {
        let (writer, guard) = NonBlockingBuilder::default().lossy(lossy).finish(file);
        tracing_subscriber::fmt()
            .with_ansi(false)
            .with_writer(writer)
            .init();
        Some(guard)
    }
}
// ANCHOR_END: start_logging

// ANCHOR: write_lines
/// What each thread does: log a hundred thousand lines, as fast as it can.
fn write_lines() {
    for line in 0..LINES {
        info!(line = line, "converting request");
    }
}
// ANCHOR_END: write_lines

// ANCHOR: main
fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let file = File::create("flood.log").unwrap();

    {
        let _guard = start_logging(mode == "sync", mode == "lossy", file);
        let start = Instant::now();
        let mut threads = Vec::new();
        for _ in 0..THREADS {
            threads.push(thread::spawn(write_lines));
        }
        for thread in threads {
            thread.join().unwrap();
        }
        println!("{mode}: the threads were done in {:?}", start.elapsed());
    }

    let written = std::fs::read_to_string("flood.log")
        .unwrap()
        .lines()
        .count();
    println!("{mode}: {written} of {} lines written", THREADS * LINES);
}
// ANCHOR_END: main
