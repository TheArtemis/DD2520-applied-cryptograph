use cryptopals::set2::oracle::{get_random_key, oracle_encrypt, oracle_guess_mode};

#[test]
fn test_challenge11_get_random_key_length() {
    let key = get_random_key();
    assert_eq!(key.len(), 16, "Key must be 16 bytes");
}

#[test]
fn test_challenge11_get_random_key_different_each_time() {
    let key1 = get_random_key();
    let key2 = get_random_key();
    assert_ne!(key1, key2, "Random keys should differ");
}


#[test]
fn test_challenge11_oracle_encrypt() {
    let plaintext = vec![b'A'; 48];
    for _ in 0..10 {
        let (ciphertext, mode) = oracle_encrypt(&plaintext, None, None);
        let guessed_mode = oracle_guess_mode(&ciphertext);
        assert_eq!(mode, guessed_mode, "Mode should be guessed correctly");
    }
}