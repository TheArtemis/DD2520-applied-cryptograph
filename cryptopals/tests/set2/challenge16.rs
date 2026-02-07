use cryptopals::set2::bitflip::{bitflip_attack, encrypt_user_data, is_admin};
use cryptopals::set2::cbc::cbc_decrypt;

#[test]
fn test_bitflip_attack() {
    let key = [0u8; 16];
    let injected_bytes = b"AAAAAAAAAAAAAAAA";

    // User input is 16 A's so the first userdata block in plaintext is "AAAAAAAAAAAAAAAA"
    let input = "AAAAAAAAAAAAAAAA";
    let ciphertext = encrypt_user_data(input, Some(&key));

    // Before attack: decrypted plaintext does not contain "admin=true"
    assert!(!is_admin(&ciphertext, &key));

    // Perform bitflip attack to inject "admin=true" into the decrypted plaintext
    let modified_ciphertext = bitflip_attack(&ciphertext, injected_bytes);

    // After attack: decrypted plaintext contains "admin=true"
    assert!(is_admin(&modified_ciphertext, &key));
}

#[test]
fn test_first_function() {
    let key = [0u8; 16];
    
    // Test with input containing special characters that should be quoted
    let input = "test;admin=true";
    let ciphertext = encrypt_user_data(input, Some(&key));
    
    // Decrypt and verify
    let decrypted = cbc_decrypt(&ciphertext, &key, None, None);
    let decrypted_str = String::from_utf8_lossy(&decrypted);
    
    // Verify prepend and append are present
    assert!(decrypted_str.starts_with("comment1=cooking%20MCs;userdata="));
    assert!(decrypted_str.ends_with(";comment2=%20like%20a%20pound%20of%20bacon"));
    
    // Verify special characters in user input are quoted
    assert!(decrypted_str.contains("test\\;admin\\=true"));
    
    // Verify ciphertext is properly padded
    assert!(!ciphertext.is_empty());
    assert_eq!(ciphertext.len() % 16, 0);
    
    // Verify is_admin returns false for legitimate encrypted data (no "admin=true" in plaintext)
    assert!(!is_admin(&ciphertext, &key));
}
