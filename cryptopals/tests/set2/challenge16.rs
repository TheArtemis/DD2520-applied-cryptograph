use cryptopals::set2::bitflip::first_function;
use cryptopals::set2::cbc::cbc_decrypt;

#[test]
fn test_first_function() {
    let key = [0u8; 16];
    
    // Test with input containing special characters that should be quoted
    let input = "test;data=value";
    let ciphertext = first_function(input, Some(&key));
    
    // Decrypt and verify
    let decrypted = cbc_decrypt(&ciphertext, &key, None);
    let decrypted_str = String::from_utf8_lossy(&decrypted);
    
    // Verify prepend and append are present
    assert!(decrypted_str.starts_with("comment1=cooking%20MCs;userdata="));
    assert!(decrypted_str.ends_with(";comment2=%20like%20a%20pound%20of%20bacon"));
    
    // Verify special characters in user input are quoted
    assert!(decrypted_str.contains("test\\;data\\=value"));
    
    // Verify ciphertext is properly padded
    assert!(!ciphertext.is_empty());
    assert_eq!(ciphertext.len() % 16, 0);
}
