
use cryptopals::set1::encoding::base64_decode;
use cryptopals::set2::cbc::{cbc_decrypt, cbc_encrypt};

const KEY: &[u8] = b"YELLOW SUBMARINE";

#[test]
fn test_challenge10_decrypt_file() {
    let base64_ciphertext = include_str!("data/challenge10.txt");
    let ciphertext = base64_decode(base64_ciphertext.trim());

    assert_eq!(
        ciphertext.len() % 16,
        0,
        "Ciphertext length must be a multiple of 16 for CBC"
    );

    let key: [u8; 16] = KEY.try_into().expect("key must be 16 bytes");
    let iv = [0u8; 16]; // All zeros

    let plaintext_bytes = cbc_decrypt(&ciphertext, &key, Some(&iv));
    let plaintext = String::from_utf8_lossy(&plaintext_bytes);

    println!("{}", plaintext);

    let cipher_text = cbc_encrypt(&plaintext_bytes, &key, Some(&iv));
    assert_eq!(
        ciphertext,
        cipher_text,
        "Encrypt then decrypt should yield original ciphertext"
    );
}

#[test]
fn test_challenge10_encrypt_then_decrypt() {
    let plaintext = b"Hello, this is a test message for CBC mode encryption!";
    let key: [u8; 16] = KEY.try_into().expect("key must be 16 bytes");
    let iv = [0u8; 16];

    let cipher_text = cbc_encrypt(plaintext, &key, Some(&iv));
    
    let decrypted = cbc_decrypt(&cipher_text, &key, Some(&iv));
    
    assert_eq!(
        plaintext,
        decrypted.as_slice(),
        "Encrypt then decrypt should yield original plaintext"
    );
}
