use crate::set1::encoding::{hex_decode, hex_encode};

pub fn hex_xor(a: &str, b: &str) -> String {
    let a_bytes = hex_decode(a);
    let b_bytes = hex_decode(b);
    let result = xor_bytes(a_bytes, b_bytes);

    hex_encode(&result)
}

pub fn xor_bytes(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    assert_eq!(a.len(), b.len(), "Bytes must be the same length");
    
    let mut result = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        result.push(a[i] ^ b[i]);
    }

    result
}

