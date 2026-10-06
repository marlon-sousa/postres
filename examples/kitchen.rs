//! Six cooks, one kitchen: the same potatoes as `potatoes.rs`, with a pinch of salt
//! in the soup every hundred potatoes. First with everything behind one lock, then
//! with the pile split beforehand and only the soup behind a lock.
//!
//! Run it with `cargo run --release --example kitchen`.

use std::sync::Mutex;
use std::thread;
use std::time::Instant;

const POTATOES: u64 = 12_000_000;
const COOKS: u64 = 6;
const SALT_EVERY: u64 = 100;

// ANCHOR: kitchen
/// Everything the cooks share: the pile, the count of primes, and the soup.
struct Kitchen {
    next_potato: u64,
    primes: u64,
    salt: u64,
}

static KITCHEN: Mutex<Kitchen> = Mutex::new(Kitchen {
    next_potato: 0,
    primes: 0,
    salt: 0,
});
// ANCHOR_END: kitchen

// ANCHOR: one_lock
/// Takes the next potato from the shared pile, peels it, and salts the soup when
/// it is time — all while holding the one lock on the whole kitchen.
fn cook_with_one_lock() {
    for _ in 0..POTATOES / COOKS {
        let mut kitchen = KITCHEN.lock().unwrap();
        let potato = kitchen.next_potato;
        kitchen.next_potato += 1;
        if is_prime(potato) {
            kitchen.primes += 1;
        }
        if potato % SALT_EVERY == 0 {
            kitchen.salt += 1;
        }
    }
}
// ANCHOR_END: one_lock

// ANCHOR: own_sack
static SOUP: Mutex<u64> = Mutex::new(0);

/// Peels a sack of potatoes nobody else touches, and takes the lock on the soup
/// only for the pinch of salt.
fn cook_with_own_sack(first: u64, last: u64) -> u64 {
    let mut primes = 0;
    for potato in first..last {
        if is_prime(potato) {
            primes += 1;
        }
        if potato % SALT_EVERY == 0 {
            let mut salt = SOUP.lock().unwrap();
            *salt += 1;
        }
    }
    primes
}
// ANCHOR_END: own_sack

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

fn main() {
    // ANCHOR: alone
    let start = Instant::now();
    let primes = cook_with_own_sack(0, POTATOES);
    let salt = *SOUP.lock().unwrap();
    println!(
        "one cook:             {primes} primes, {salt} pinches of salt, in {:?}",
        start.elapsed()
    );
    // ANCHOR_END: alone

    *SOUP.lock().unwrap() = 0;

    // ANCHOR: with_one_lock
    let start = Instant::now();
    let mut cooks = Vec::new();
    for _ in 0..COOKS {
        cooks.push(thread::spawn(cook_with_one_lock));
    }
    for cook in cooks {
        cook.join().unwrap();
    }
    let kitchen = KITCHEN.lock().unwrap();
    println!(
        "six cooks, one lock:  {} primes, {} pinches of salt, in {:?}",
        kitchen.primes,
        kitchen.salt,
        start.elapsed()
    );
    // ANCHOR_END: with_one_lock

    // ANCHOR: with_own_sacks
    let start = Instant::now();
    let mut cooks = Vec::new();
    let sack = POTATOES / COOKS;
    for cook in 0..COOKS {
        let first = cook * sack;
        let hired = thread::spawn(move || cook_with_own_sack(first, first + sack));
        cooks.push(hired);
    }
    let mut primes = 0;
    for cook in cooks {
        primes += cook.join().unwrap();
    }
    let salt = *SOUP.lock().unwrap();
    println!(
        "six cooks, own sacks: {primes} primes, {salt} pinches of salt, in {:?}",
        start.elapsed()
    );
    // ANCHOR_END: with_own_sacks
}
