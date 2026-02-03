use crate::set1::encoding::hex_decode;

pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    let mut distance = 0;
    assert_eq!(a.len(), b.len(), "Inputs must have the same length");
    for i in 0..a.len() {
        distance += (a[i] ^ b[i]).count_ones() as usize;
    }
    distance
}

pub fn hamming_distance_hex(a: &str, b: &str) -> usize {
    let a_bytes = hex_decode(a);
    let b_bytes = hex_decode(b);
    hamming_distance(&a_bytes, &b_bytes)
}