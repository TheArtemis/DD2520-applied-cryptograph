
use cryptopals::set2::cbc::cbc_decrypt;
use cryptopals::set2::cbc::cbc_encrypt;

const KEY: &[u8] = b"YELLOW SUBMARINE";


#[test]
fn test_challenge10_encrypt_then_decrypt() {
    let plaintext = b"Hello, this is a test message for CBC mode encryption!";
    let key: [u8; 16] = KEY.try_into().expect("key must be 16 bytes");
    let iv = [0u8; 16];

    let cipher_text = cbc_encrypt(plaintext, &key, Some(&iv));
    
    let decrypted = cbc_decrypt(&cipher_text, &key, Some(&iv));
    
    println!("{}", String::from_utf8_lossy(&decrypted));

    assert_eq!(
        plaintext,
        decrypted.as_slice(),
        "Encrypt then decrypt should yield original plaintext"
    );
}
