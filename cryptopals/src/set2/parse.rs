use std::collections::HashMap;
use crate::set2::{aes_128_ecb_encrypt, aes_128_ecb_decrypt, padding::pkcs7_pad, padding::pkcs7_unpad};

pub fn parse_kv(str: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in str.split('&') {
        let parts = pair.splitn(2, '=').collect::<Vec<&str>>();
        if parts.len() == 2 {
            map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    map
}

pub fn profile_for(email: &str) -> String {
    // Sanitize email by removing metacharacters (& and =)
    let sanitized_email: String = email.chars().filter(|c| *c != '&' && *c != '=').collect();
    
    format!("email={}&uid=10&role=user", sanitized_email)
}

pub fn encrypt_profile(email: &str, key: &[u8; 16]) -> Vec<u8> {
    let profile = profile_for(email);
    let profile_bytes = profile.as_bytes();
    let padded = pkcs7_pad(profile_bytes, 16);
    aes_128_ecb_encrypt(&padded, key)
}

pub fn decrypt_and_parse_profile(ciphertext: &[u8], key: &[u8; 16]) -> HashMap<String, String> {
    let decrypted = aes_128_ecb_decrypt(ciphertext, key);
    let unpadded = pkcs7_unpad(&decrypted);
    let profile_str = String::from_utf8_lossy(&unpadded);
    parse_kv(&profile_str)
}