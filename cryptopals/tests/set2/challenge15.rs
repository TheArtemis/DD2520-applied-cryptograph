use cryptopals::set2::padding::{pkcs7_validate, pkcs7_validate_str, pkcs7_unpad, pkcs7_unpad_str};

#[test]
fn test_valid_padding() {
    // "ICE ICE BABY\x04\x04\x04\x04" has valid padding
    let valid_padded = "ICE ICE BABY\x04\x04\x04\x04";
    assert!(pkcs7_validate_str(valid_padded));
    assert!(pkcs7_validate(valid_padded.as_bytes()));
    
    // Should strip to "ICE ICE BABY"
    let unpadded = pkcs7_unpad_str(valid_padded);
    assert_eq!(unpadded, "ICE ICE BABY");
}

#[test]
fn test_invalid_padding_wrong_value() {
    // "ICE ICE BABY\x05\x05\x05\x05" does not have valid padding
    // (padding value doesn't match the number of padding bytes)
    let invalid_padded = "ICE ICE BABY\x05\x05\x05\x05";
    assert!(!pkcs7_validate_str(invalid_padded));
    assert!(!pkcs7_validate(invalid_padded.as_bytes()));
}

#[test]
fn test_invalid_padding_mismatched_bytes() {
    // "ICE ICE BABY\x01\x02\x03\x04" does not have valid padding
    // (padding bytes are not all the same)
    let invalid_padded = "ICE ICE BABY\x01\x02\x03\x04";
    assert!(!pkcs7_validate_str(invalid_padded));
    assert!(!pkcs7_validate(invalid_padded.as_bytes()));
}

#[test]
fn test_empty_string() {
    // Empty string should return true (edge case)
    assert!(pkcs7_validate_str(""));
    assert!(pkcs7_validate(&[]));
}

#[test]
fn test_unpad_valid_padding() {
    let valid_padded = "ICE ICE BABY\x04\x04\x04\x04";
    let unpadded_bytes = pkcs7_unpad(valid_padded.as_bytes());
    assert_eq!(unpadded_bytes, b"ICE ICE BABY");
}

#[test]
fn test_unpad_invalid_padding() {
    // When padding is invalid, unpad should return the original data unchanged
    let invalid_padded = "ICE ICE BABY\x05\x05\x05\x05";
    let unpadded_bytes = pkcs7_unpad(invalid_padded.as_bytes());
    assert_eq!(unpadded_bytes, invalid_padded.as_bytes());
}
