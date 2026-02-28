// Challenge 24: MT19937 stream cipher and break it.
//
// - Stream cipher: keystream = 8-bit outputs from MT19937(seed), XOR with data (16-bit seed).
// - Encrypt known plaintext (14 'A's) with random prefix, brute-force recover the seed.
// - Password reset token = MT19937(current_time).next_u32(); check if a token could be from that.

use cryptopals::set3::mt19937_stream::{
    is_token_from_mt19937_current_time, is_token_from_mt19937_seed_range,
    mt19937_stream_decrypt, mt19937_stream_encrypt, password_reset_token_from_seed,
    password_reset_token_now, recover_seed_from_suffix,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_mt19937_stream_encrypt_decrypt() {
    let seed = 0x1234u16;
    let plaintext = b"Hello, MT19937 stream cipher!";
    let ciphertext = mt19937_stream_encrypt(plaintext, seed);
    let decrypted = mt19937_stream_decrypt(&ciphertext, seed);
    assert_eq!(plaintext.as_slice(), decrypted.as_slice());
}

#[test]
fn test_recover_seed_known_plaintext_suffix() {
    let known_suffix = [b'A'; 14];
    let seed = 0xABCDu16; // arbitrary 16-bit seed

    // Random-length random prefix (fixed in test for reproducibility)
    let prefix_len = 7;
    let prefix: Vec<u8> = (0..prefix_len).map(|i| (i as u8).wrapping_mul(31)).collect();
    let plaintext: Vec<u8> = prefix.iter().chain(known_suffix.iter()).copied().collect();

    let ciphertext = mt19937_stream_encrypt(&plaintext, seed);
    let recovered = recover_seed_from_suffix(&ciphertext, &known_suffix);
    assert_eq!(recovered, Some(seed));
}

#[test]
fn test_recover_seed_random_prefix_lengths() {
    let known_suffix = [b'A'; 14];
    for &seed in &[0u16, 1u16, 0xFFu16, 0x1234u16, 0xFFFFu16] {
        for prefix_len in [0, 1, 5, 20, 100] {
            let prefix: Vec<u8> = (0..prefix_len).map(|i| (i as u8).wrapping_mul(17)).collect();
            let plaintext: Vec<u8> = prefix.iter().chain(known_suffix.iter()).copied().collect();
            let ciphertext = mt19937_stream_encrypt(&plaintext, seed);
            let recovered = recover_seed_from_suffix(&ciphertext, &known_suffix);
            assert_eq!(recovered, Some(seed), "seed={} prefix_len={}", seed, prefix_len);
        }
    }
}

#[test]
fn test_password_reset_token_from_seed_and_check() {
    let now_ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    let token = password_reset_token_from_seed(now_ts);
    assert!(is_token_from_mt19937_seed_range(token, now_ts, now_ts));

    let other_ts = now_ts.wrapping_sub(100);
    let wrong_token = password_reset_token_from_seed(other_ts);
    assert!(is_token_from_mt19937_seed_range(wrong_token, other_ts, other_ts));
    assert!(!is_token_from_mt19937_seed_range(wrong_token, now_ts, now_ts));
}

#[test]
fn test_is_token_from_mt19937_current_time() {
    let token = password_reset_token_now();
    assert!(
        is_token_from_mt19937_current_time(token, 2),
        "token just generated should be recognized as from current time within 2s window"
    );
    let random_token = 0xDEADBEEFu32;
    assert!(
        !is_token_from_mt19937_current_time(random_token, 1),
        "random value should not be accepted (unless extremely unlikely collision)"
    );
}
