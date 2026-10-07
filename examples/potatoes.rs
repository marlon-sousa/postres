//! Peeling potatoes: the same work done by one thread, then shared among six,
//! each with a sack of its own.
//!
//! Each "potato" is one number to check for being prime: a small job, repeated
//! millions of times, and no potato depends on any other.
//!
//! Run it with `cargo run --release --example potatoes`.

use std::thread;
use std::time::Instant;

// ANCHOR: consts
const POTATOES: u64 = 12_000_000;
const COOKS: u64 = 6;
// ANCHOR_END: consts

// ANCHOR: peel
/// Peels the potatoes from `first` up to, but not including, `last`, and says how
/// many of them turned out to be prime.
fn peel(first: u64, last: u64) -> u64 {
    let mut primes = 0;
    for potato in first..last {
        if is_prime(potato) {
            primes += 1;
        }
    }
    primes
}
// ANCHOR_END: peel

// ANCHOR: is_prime
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut divisor = 2;
    while divisor * divisor <= n {
        if n % divisor == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}
// ANCHOR_END: is_prime

fn main() {
    // ANCHOR: alone
    let start = Instant::now();
    let primes = peel(0, POTATOES);
    println!("one cook:  {primes} primes in {:?}", start.elapsed());
    // ANCHOR_END: alone

    // ANCHOR: together
    let start = Instant::now();
    let mut cooks = Vec::new();
    let sack = POTATOES / COOKS;
    for cook in 0..COOKS {
        let first = cook * sack;
        let hired = thread::spawn(move || peel(first, first + sack));
        cooks.push(hired);
    }
    let mut primes = 0;
    for cook in cooks {
        primes += cook.join().unwrap();
    }
    println!("six cooks: {primes} primes in {:?}", start.elapsed());
    // ANCHOR_END: together
}
