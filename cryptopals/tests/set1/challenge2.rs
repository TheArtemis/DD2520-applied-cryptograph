use cryptopals::set1::xor::hex_xor;

const INPUT_A: &str = "1c0111001f010100061a024b53535009181c";
const INPUT_B: &str = "686974207468652062756c6c277320657965";
const EXPECTED_OUTPUT: &str = "746865206b696420646f6e277420706c6179";

#[test]
fn test_challenge2() {
    let output = hex_xor(INPUT_A, INPUT_B);
    assert_eq!(output, EXPECTED_OUTPUT);
}