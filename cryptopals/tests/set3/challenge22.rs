// Challenge 22: Timestamp-seeded MT19937 — simulate routine that waits, seeds with time, returns
// first output; crack the seed.

use cryptopals::set3::{
    mt19937::Mt19937,
    mt19937_attack::{crack_seed_from_first_output, first_output_for_seed},
};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// Runs the full routine: wait 40–1000s, seed with current Unix timestamp, wait again, return (first_output, seed_used).
/// Uses thread::sleep; for tests prefer first_output_for_seed with a chosen timestamp.
#[allow(dead_code)]
pub fn run_timestamp_rng_with_waits() -> (u32, u32) {
    let mut rng = rand::rngs::OsRng;
    let wait1: u64 = rng.gen_range(40..=1000);
    std::thread::sleep(std::time::Duration::from_secs(wait1));

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs() as u32;

    let wait2: u64 = rng.gen_range(40..=1000);
    std::thread::sleep(std::time::Duration::from_secs(wait2));

    let output = first_output_for_seed(seed);
    (output, seed)
}

#[test]
fn test_mt19937_same_seed_same_sequence() {
    let mut a = Mt19937::new(12345);
    let mut b = Mt19937::new(12345);
    for _ in 0..100 {
        assert_eq!(a.next_u32(), b.next_u32(), "same seed must give same sequence");
    }
}

#[test]
fn test_crack_timestamp_seed() {
    // Simulate: pick a timestamp (e.g. "current" or a fixed one), get first output, then crack.
    let seed = 1_700_000_000; // example timestamp
    let output = first_output_for_seed(seed);

    // Cracker searches a window around when we might have seeded (e.g. ±2000 seconds).
    let found = crack_seed_from_first_output(output, seed - 2000, seed + 100).unwrap();
    assert_eq!(found, seed, "cracker must recover the timestamp seed");
}
