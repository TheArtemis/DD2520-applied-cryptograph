use cryptopals::set3::utils::{check_padding, get_random_line, random_cbc_encrypt, LINES};

#[test]
fn test_get_random_line() {
    // get_random_line returns one of the LINES
    let line = get_random_line();
    assert!(LINES.contains(&line.as_str()));
    assert!(!line.is_empty());
}

#[test]
fn test_padding_oracle_valid_ciphertext() {
    // First function: encrypt one of the strings; second function: check padding returns true
    let key = [0u8; 16];
    let iv = [0u8; 16];
    let ciphertext = random_cbc_encrypt(&key, &iv);
    assert!(check_padding(&ciphertext, &key, &iv));
}
