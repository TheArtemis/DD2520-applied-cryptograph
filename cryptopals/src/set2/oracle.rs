use std::collections::{HashSet};

use rand::{Rng, RngCore};
use crate::set2::{aes_128_ecb_encrypt, cbc::cbc_encrypt};

#[derive(PartialEq, Eq, Debug)]
pub enum Mode {
    ECB,
    CBC,
}

pub fn get_random_key() -> [u8; 16] {
    let mut key = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

pub fn get_random_iv() -> [u8; 16] {
    get_random_key()
}

pub fn get_random_bytes() -> Vec<u8> {
    // 5-10 random bytes
    let mut bytes = Vec::new();
    for _ in 0..rand::thread_rng().gen_range(5..=10) {
        bytes.push(rand::thread_rng().gen_range(0..=255));
    }
    bytes
}

pub fn get_random_mode() -> Mode {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        Mode::ECB
    } else {
        Mode::CBC
    }
}

pub fn oracle_encrypt(
    plaintext: &[u8],
    key: Option<[u8; 16]>,
    mode: Option<Mode>,
) -> (Vec<u8>, Mode) {
    let key = key.unwrap_or_else(get_random_key);

    let prepending_bytes = get_random_bytes();
    let appending_bytes = get_random_bytes();

    let mut combined_bytes = prepending_bytes;
    combined_bytes.extend(plaintext);
    combined_bytes.extend(appending_bytes);

    let mode = mode.unwrap_or_else(get_random_mode);
    let ciphertext = match mode {
        Mode::ECB => aes_128_ecb_encrypt(&combined_bytes, &key),
        Mode::CBC => {
            let iv = get_random_iv();
            cbc_encrypt(&combined_bytes, &key, Some(&iv))
        }
    };
    (ciphertext, mode)
}

pub fn ecb_oracle_fixed(
    plaintext: &[u8],
    key: [u8; 16],
    secret: &[u8],
) -> Vec<u8> {
    let mut combined = Vec::new();
    combined.extend(plaintext);
    combined.extend(secret);

    aes_128_ecb_encrypt(&combined, &key)
}

pub fn oracle_guess_mode(
    ciphertext: &[u8],
) -> Mode {
    let blocks: Vec<&[u8]> = ciphertext.chunks_exact(16).collect();
    
    // Check for duplicate blocks (ECB mode produces identical ciphertext blocks for identical plaintext blocks)
    let mut seen_blocks = HashSet::new();
    for block in blocks {
        let block_vec: Vec<u8> = block.to_vec();
        if seen_blocks.contains(&block_vec) {
            return Mode::ECB;
        }
        seen_blocks.insert(block_vec);
    }

    Mode::CBC
}