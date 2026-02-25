use crate::set3::mt19937::Mt19937;

/// Seed MT19937 with the given 32-bit value (e.g. Unix timestamp) and return the first 32-bit output.
pub fn first_output_for_seed(seed: u32) -> u32 {
    let mut rng = Mt19937::new(seed);
    rng.next_u32()
}

/// Crack the seed from a single first 32-bit output by brute-forcing over [seed_min, seed_max].
/// Intended for scenarios where the seed space is small (e.g. recent Unix timestamps).
pub fn crack_seed_from_first_output(
    first_output: u32,
    seed_min: u32,
    seed_max: u32,
) -> Option<u32> {
    for seed in seed_min..=seed_max {
        if first_output_for_seed(seed) == first_output {
            return Some(seed);
        }
    }
    None
}

