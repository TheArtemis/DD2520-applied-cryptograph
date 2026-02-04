//! Set 1 Challenge 7: AES in ECB mode.
//! Decrypt the base64-encoded file with key "YELLOW SUBMARINE" using our AES inv_cipher.

use aes::{AES128, State};
use cryptopals::set1::encoding::base64_decode;

const KEY: &[u8] = b"YELLOW SUBMARINE";

fn aes_128_ecb_decrypt(cipher_text: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let aes = AES128::new(*key);
    let mut plaintext = Vec::with_capacity(cipher_text.len());
    for block in cipher_text.chunks_exact(16) {
        let mut state = State::new(block.try_into().unwrap());
        aes.inv_cipher(&mut state);
        plaintext.extend_from_slice(state.as_bytes());
    }
    plaintext
}

#[test]
fn test_challenge7_decrypt_aes_ecb() {
    let base64_cipher_text = include_str!("data/challenge7.txt");
    let cipher_text = base64_decode(base64_cipher_text.trim());

    assert_eq!(
        cipher_text.len() % 16,
        0,
        "Cipher text length must be a multiple of 16 for ECB"
    );

    let key: [u8; 16] = KEY.try_into().expect("key must be 16 bytes");
    let plaintext_bytes = aes_128_ecb_decrypt(&cipher_text, &key);
    let plaintext = String::from_utf8_lossy(&plaintext_bytes);

    println!("{}", plaintext);

    // Same plaintext as challenge 6 (Vanilla Ice lyrics)
    assert!(
        plaintext.contains("I'm back"),
        "Decrypted text should contain 'I'm back', got: {}",
        &plaintext[..plaintext.len().min(200)]
    );
}
