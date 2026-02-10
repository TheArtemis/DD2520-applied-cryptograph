use crate::{AES128, State};

/// Decrypt ciphertext with AES-128 in ECB mode.
/// `ciphertext.len()` must be a multiple of 16.
pub fn aes_128_ecb_decrypt(ciphertext: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let aes = AES128::new(*key);
    let mut plaintext = Vec::with_capacity(ciphertext.len());
    for block in ciphertext.chunks_exact(16) {
        let mut state = State::new(block.try_into().unwrap());
        aes.inv_cipher(&mut state);
        plaintext.extend_from_slice(state.as_bytes());
    }
    plaintext
}

/// Encrypt plaintext with AES-128 in ECB mode.
/// `plaintext.len()` must be a multiple of 16.
pub fn aes_128_ecb_encrypt(plaintext: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let aes = AES128::new(*key);
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    for block in plaintext.chunks_exact(16) {
        let mut state = State::new(block.try_into().unwrap());
        aes.cipher(&mut state);
        ciphertext.extend_from_slice(state.as_bytes());
    }
    ciphertext
}
