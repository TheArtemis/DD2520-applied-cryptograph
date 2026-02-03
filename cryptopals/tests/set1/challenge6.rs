use cryptopals::set1::brute_force::{decrypt_repeating_key_xor_base64};
use cryptopals::set1::distance::hamming_distance;

const HAMMING_INPUT: &str = "this is a test";
const HAMMING_KEY: &str = "wokka wokka!!!";
const HAMMING_EXPECTED: usize = 37;

#[test]
fn test_challenge6_hamming_distance() {
    let output = hamming_distance(HAMMING_INPUT.as_bytes(), HAMMING_KEY.as_bytes());
    assert_eq!(output, HAMMING_EXPECTED);
}

#[test]
fn test_challenge6_decrypt() {
    let base64_cipher_text = include_str!("data/challenge6.txt");
    let plaintext: String = decrypt_repeating_key_xor_base64(base64_cipher_text);

    println!("{}", plaintext);

    // Challenge 6 plaintext starts with Vanilla Ice lyrics
    assert!(
        plaintext.contains("I'm back"),
        "Decrypted text should contain 'I'm back', got: {}",
        &plaintext[..plaintext.len().min(100)]
    );
}
