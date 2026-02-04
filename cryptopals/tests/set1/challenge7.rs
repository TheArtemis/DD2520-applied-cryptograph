use aes::aes_128_ecb_decrypt;
use cryptopals::set1::encoding::base64_decode;

const KEY: &[u8] = b"YELLOW SUBMARINE";

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

    assert!(
        plaintext.contains("I'm back"),
        "Decrypted text should contain 'I'm back', got: {}",
        &plaintext[..plaintext.len().min(200)]
    );
}
