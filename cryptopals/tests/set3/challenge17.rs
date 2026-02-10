use cryptopals::set1::encoding::base64_decode;
use cryptopals::set2::cbc::cbc_encrypt;
use cryptopals::set3::padding_oracle::padding_oracle_attack;
use cryptopals::set3::utils::{check_padding, get_random_line, random_cbc_encrypt, PaddingOracle, LINES};

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

#[test]
fn test_padding_oracle_attack() {
    // Encrypt a known plaintext; oracle holds the key (attack never sees it)
    let key = [0u8; 16];
    let iv = [0u8; 16];
    let plaintext = base64_decode(LINES[0]);
    let ciphertext = cbc_encrypt(&plaintext, &key, Some(&iv));

    let oracle = PaddingOracle::new(key);
    let recovered = padding_oracle_attack(&ciphertext, &iv, &oracle);

    assert_eq!(recovered, plaintext);
}

#[test]
fn test_padding_oracle_attack_random_line() {
    // Attack recovers plaintext when oracle picks a random line (multi-block, padding in last block)
    let key = [0u8; 16];
    let iv = [0u8; 16];
    let ciphertext = random_cbc_encrypt(&key, &iv);
    let oracle = PaddingOracle::new(key);

    let recovered = padding_oracle_attack(&ciphertext, &iv, &oracle);

    // Recovered (after unpad) must equal one of the decoded LINES
    let decoded_lines: Vec<Vec<u8>> = LINES.iter().map(|s| base64_decode(s)).collect();
    assert!(
        decoded_lines.contains(&recovered),
        "recovered plaintext should be one of the LINES"
    );
}
