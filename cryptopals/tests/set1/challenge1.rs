use cryptopals::set1::encoding::hex_to_base64;

const INPUT: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const EXPECTED_OUTPUT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

#[test]
fn test_challenge1() {
    let output = hex_to_base64(INPUT);
    assert_eq!(output, EXPECTED_OUTPUT);
}