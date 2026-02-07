use rand::Rng;

use crate::{set1::encoding::base64_decode, set2::{cbc::{cbc_decrypt, cbc_encrypt}, padding::pkcs7_validate}};

pub const LINES: [&str; 10] = ["MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
"MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
"MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
"MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
"MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
"MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
"MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
"MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
"MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
"MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"];

pub fn get_random_line() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..LINES.len());
    String::from(LINES[index])
}

pub fn random_cbc_encrypt(key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    let plaintext = get_random_line();
    let bytes = base64_decode(plaintext.as_str());
    let ciphertext = cbc_encrypt(&bytes, key, Some(iv));
    ciphertext
}

/// Oracle: decrypt and check padding. Must validate the raw decrypted plaintext
/// (before stripping padding), otherwise the padding oracle is wrong.
pub fn check_padding(ciphertext: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> bool {
    let raw_plaintext = cbc_decrypt(ciphertext, key, Some(iv), Some(false));
    pkcs7_validate(&raw_plaintext)
}