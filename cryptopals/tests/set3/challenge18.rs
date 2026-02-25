use cryptopals::{set1::encoding::base64_decode, set3::ctr::{ctr_decrypt, ctr_encrypt}};

const INPUT_BASE64: &str = "L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==";
const KEY: &[u8; 16] = b"YELLOW SUBMARINE";
const NONCE: &[u8; 8] = b"00000000";

#[test]
fn test_ctr_encrypt_decrypt() {
    let plaintext = base64_decode(INPUT_BASE64);
    println!("Plaintext: {:?}", plaintext);
    let ciphertext = ctr_encrypt(&plaintext, KEY, NONCE);
    let decrypted = ctr_decrypt(&ciphertext, KEY, NONCE);
    assert_eq!(plaintext, decrypted);
}
