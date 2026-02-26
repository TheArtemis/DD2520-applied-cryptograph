use crate::set3::mt19937::{Mt19937, N};

/// Inverse of y ^= y >> 18: same operation, y = z ^ (z >> 18).
fn untemper_right18(z: u32) -> u32 {
    z ^ (z >> 18)
}

/// Inverse of y ^= (y << 15) & C: same operation, y = z ^ ((z << 15) & 0xEFC6_0000).
fn untemper_left15(z: u32) -> u32 {
    z ^ ((z << 15) & 0xEFC6_0000)
}

/// Inverse of y ^= (y << 7) & B: recover in chunks of 7 bits (multiple masks).
fn untemper_left7(z: u32) -> u32 {
    const B: u32 = 0x9D2C_5680;
    const S: u32 = 7;
    let mut y = z;
    let smask = (1 << S) - 1;
    y ^= (y << S) & B & (smask << S);
    y ^= (y << S) & B & (smask << (S * 2));
    y ^= (y << S) & B & (smask << (S * 3));
    y ^= (y << S) & B & (smask << (S * 4));
    y
}

/// Inverse of y ^= y >> 11: recover in chunks of 11 bits (multiple masks).
fn untemper_right11(z: u32) -> u32 {
    const U: u32 = 11;
    let umask = (1 << U) - 1;
    let mut y = z;
    y ^= (y >> U) & (umask << (U * 2));
    y ^= (y >> U) & (umask << U);
    y ^= (y >> U) & umask;
    y
}

/// Reverse the MT19937 tempering transform to recover the internal state element from an output.
/// Order: invert last step first (>>18), then <<15&M, then <<7&M, then >>11.
pub fn untemper(y: u32) -> u32 {
    let y = untemper_right18(y);
    let y = untemper_left15(y);
    let y = untemper_left7(y);
    untemper_right11(y)
}

/// Recreate an MT19937 from 624 consecutive outputs by untempering each and building the state.
/// The returned generator will produce the same subsequent outputs as the original.
pub fn clone_from_outputs(outputs: &[u32; N]) -> Mt19937 {
    let mut mt = [0u32; N];
    for (i, &out) in outputs.iter().enumerate() {
        mt[i] = untemper(out);
    }
    Mt19937::from_state(mt)
}

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

