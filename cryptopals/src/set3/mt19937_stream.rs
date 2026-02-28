// MT19937 stream cipher: keystream from MT19937 (16-bit seed), XOR with data.
// Same pattern as CTR: one function for keystream XOR, encrypt/decrypt are identical.

use crate::{set1::xor::xor_bytes, set3::mt19937::Mt19937};

/// Generate keystream bytes from MT19937 seeded with the given 16-bit seed.
/// Each keystream byte is the low 8 bits of a 32-bit MT output (sequence of 8-bit outputs).
fn mt19937_keystream(seed: u16, len: usize) -> Vec<u8> {
    let mut rng = Mt19937::new(seed as u32);
    (0..len).map(|_| (rng.next_u32() & 0xFF) as u8).collect()
}

/// XOR input with MT19937 keystream (16-bit seed). Used for both encrypt and decrypt.
fn mt19937_stream_xor(input: &[u8], seed: u16) -> Vec<u8> {
    let keystream = mt19937_keystream(seed, input.len());
    xor_bytes(input, &keystream)
}

/// Encrypt plaintext with MT19937 stream cipher using the given 16-bit seed.
pub fn mt19937_stream_encrypt(plaintext: &[u8], seed: u16) -> Vec<u8> {
    mt19937_stream_xor(plaintext, seed)
}

/// Decrypt ciphertext with MT19937 stream cipher using the given 16-bit seed.
pub fn mt19937_stream_decrypt(ciphertext: &[u8], seed: u16) -> Vec<u8> {
    mt19937_stream_xor(ciphertext, seed)
}

/// Recover the 16-bit MT19937 stream cipher seed by brute force.
/// Assumes `ciphertext` is encrypt(random_prefix || known_suffix) and we know `known_suffix`.
/// Returns the seed that makes the last `known_suffix.len()` keystream bytes match
/// ciphertext_suffix XOR known_suffix.
pub fn recover_seed_from_suffix(ciphertext: &[u8], known_suffix: &[u8]) -> Option<u16> {
    if known_suffix.is_empty() || ciphertext.len() < known_suffix.len() {
        return None;
    }
    let start = ciphertext.len() - known_suffix.len();
    let ct_suffix = &ciphertext[start..];
    let expected_keystream: Vec<u8> = ct_suffix
        .iter()
        .zip(known_suffix.iter())
        .map(|(c, p)| c ^ p)
        .collect();

    for seed in 0u16..=0xFFFF {
        let mut rng = Mt19937::new(seed as u32);
        // Advance keystream to the position of the suffix
        for _ in 0..start {
            rng.next_u32();
        }
        let mut match_ = true;
        for &expected in &expected_keystream {
            if (rng.next_u32() & 0xFF) as u8 != expected {
                match_ = false;
                break;
            }
        }
        if match_ {
            return Some(seed);
        }
    }
    None
}

/// Generate a password reset token as the first u32 output of MT19937 seeded with `seed` (e.g. current time).
pub fn password_reset_token_from_seed(seed: u32) -> u32 {
    Mt19937::new(seed).next_u32()
}

/// Check whether `token` could be the first output of MT19937 seeded with any timestamp in [start_ts, end_ts].
pub fn is_token_from_mt19937_seed_range(token: u32, start_ts: u32, end_ts: u32) -> bool {
    for seed in start_ts..=end_ts {
        if Mt19937::new(seed).next_u32() == token {
            return true;
        }
    }
    false
}

/// Generate a password reset token using MT19937 seeded with the current Unix timestamp.
pub fn password_reset_token_now() -> u32 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    password_reset_token_from_seed(now)
}

/// Check if `token` is the first output of MT19937 seeded with the current time (within `window_secs`).
pub fn is_token_from_mt19937_current_time(token: u32, window_secs: u32) -> bool {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    let start = now.saturating_sub(window_secs);
    is_token_from_mt19937_seed_range(token, start, now)
}
