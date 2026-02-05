use cryptopals::set1::encoding::base64_decode;
use cryptopals::set2::bat::bat_decrypt;
use cryptopals::set2::oracle::ecb_oracle_fixed;

const BASE64_SECRET: &str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
YnkK";

#[test]
fn test_challenge12_byte_at_a_time_ecb_decrypt() {
    let secret = base64_decode(BASE64_SECRET);
    let key = [0u8; 16]; // fixed key for deterministic test

    let oracle = |input: &[u8]| ecb_oracle_fixed(input, key, &secret);
    let recovered = bat_decrypt(oracle);

    println!("Recovered: {:?}", String::from_utf8_lossy(&recovered));

    assert_eq!(recovered, secret, "Recovered plaintext should match the secret");
}
