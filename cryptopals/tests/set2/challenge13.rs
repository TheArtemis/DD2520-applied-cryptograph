use aes::aes_128_ecb_encrypt;
use cryptopals::set2::parse::{parse_kv, profile_for, encrypt_profile, decrypt_and_parse_profile};
use cryptopals::set2::oracle::get_random_key;
use cryptopals::set2::padding::pkcs7_pad;
use std::collections::HashMap;

#[test]
fn test_parse_kv() {
    let input = "foo=bar&baz=qux&zap=zazzle";
    let result = parse_kv(input);

    let mut expected = HashMap::new();
    expected.insert("foo".to_string(), "bar".to_string());
    expected.insert("baz".to_string(), "qux".to_string());
    expected.insert("zap".to_string(), "zazzle".to_string());

    assert_eq!(result, expected);
}

#[test]
fn test_profile_for() {
    let result = profile_for("foo@bar.com");
    assert_eq!(result, "email=foo@bar.com&uid=10&role=user");
}

#[test]
fn test_ecb_cut_and_paste_attack() {
    let key = get_random_key();
    
    // Craft email1 so "role=" ends at block boundary (32 bytes)
    // "email=" (6) + email + "&uid=10&role=" (11) = 32 bytes
    // So email_len = 15
    let email1 = "a".repeat(10) + "@xxx."; // 15 bytes
    println!("Email1: {}", email1);
    let ciphertext1 = encrypt_profile(&email1, &key);
    
    // Create profile manually with "admin" to get the admin block
    let email2 = "a".repeat(10) + "@xxx."; // 15 bytes
    let profile2 = format!("email={}&uid=10&role=admin", email2);
    let padded2 = pkcs7_pad(profile2.as_bytes(), 16);
    let ciphertext2 = aes_128_ecb_encrypt(&padded2, &key);
    
    // Cut and paste: first 2 blocks from ciphertext1, last block from ciphertext2
    let mut attack_ciphertext = Vec::new();
    // "email=aaaaaaaaaa@xxx.&uid=10&role="
    attack_ciphertext.extend_from_slice(&ciphertext1[0..32]);

    // "adminPKCS7Padding"
    attack_ciphertext.extend_from_slice(&ciphertext2[32..48]);
    
    // Decrypt and verify we got role=admin
    let profile = decrypt_and_parse_profile(&attack_ciphertext, &key);
    println!("Profile: {:?}", profile);
    assert_eq!(profile.get("role"), Some(&"admin".to_string()));
}
