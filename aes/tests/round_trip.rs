//! Test: encrypt a block, then decrypt it, and check that output equals input.

use aes::{AES128, State};

#[test]
fn encrypt_then_decrypt_equals_original() {
    let key = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
    let input: [u8; 16] = [0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];

    let aes = AES128::new(key);
    let mut state = State::new(input);

    aes.cipher(&mut state);
    aes.inv_cipher(&mut state);

    let output = *state.as_bytes();
    assert_eq!(input, output, "encrypt then decrypt must yield original input");
}

#[test]
fn encrypt_then_decrypt_another_block() {
    let key = [0u8; 16]; // all-zero key
    let input: [u8; 16] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];

    let aes = AES128::new(key);
    let mut state = State::new(input);

    aes.cipher(&mut state);
    aes.inv_cipher(&mut state);

    let output = *state.as_bytes();
    assert_eq!(input, output, "encrypt then decrypt must yield original input");
}
