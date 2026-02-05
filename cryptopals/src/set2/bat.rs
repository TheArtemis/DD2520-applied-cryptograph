use std::collections::HashSet;

use crate::set2::padding::pkcs7_unpad;

pub fn bat_decrypt<F>(oracle: F) -> Vec<u8>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    let block_size = guess_block_size(&oracle);

    let test = oracle(&vec![b'A'; block_size * 4]);

    // Check if the oracle is using ECB mode
    assert!(is_ecb(&test, block_size));

    let mut recovered = Vec::new();

    loop {
        // Get the block index and the padding length
        let block_index = recovered.len() / block_size;
        let pad_len = block_size - (recovered.len() % block_size) - 1;

        // Create the prefix    
        let prefix = vec![b'A'; pad_len];
        let ciphertext = oracle(&prefix);

        let start = block_index * block_size;
        let end = start + block_size;
        if end > ciphertext.len() {
            break;
        }

        let target_block = &ciphertext[start..end];
        let mut found = false;

        for byte in 0u8..=255 {
            let mut attempt = prefix.clone();
            attempt.extend(&recovered);
            attempt.push(byte);

            let attempt_cipher = oracle(&attempt);
            let attempt_block = &attempt_cipher[start..end];

            if attempt_block == target_block {
                recovered.push(byte);
                found = true;
                break;
            }
        }

        if !found {
            break;
        }
    }

    pkcs7_unpad(&recovered)
}

pub fn is_ecb(ciphertext: &[u8], block_size: usize) -> bool {
    let mut seen = HashSet::new();
    for block in ciphertext.chunks(block_size) {
        if !seen.insert(block.to_vec()) {
            return true;
        }
    }
    false
}

pub fn guess_block_size<F>(oracle: F) -> usize
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    let base_len = oracle(&[]).len();

    for i in 1..=64 {
        let input = vec![b'A'; i];
        let new_len = oracle(&input).len();
        if new_len > base_len {
            return new_len - base_len;
        }
    }

    panic!("Block size not found");
}
