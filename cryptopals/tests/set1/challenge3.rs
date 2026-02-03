use cryptopals::set1::brute_force::decrypt_single_byte_encrypted_hex;

const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
const EXPECTED_OUTPUT: &str = "Cooking MC's like a pound of bacon";


#[test]
fn test_challenge3() {
    let output = decrypt_single_byte_encrypted_hex(INPUT);
    assert_eq!(output, EXPECTED_OUTPUT);
}