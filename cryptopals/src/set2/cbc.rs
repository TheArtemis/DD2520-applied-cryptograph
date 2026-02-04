use crate::{set1::xor::xor_bytes, set2::padding::{pkcs7_pad, pkcs7_unpad}, set2::{aes_128_ecb_encrypt, aes_128_ecb_decrypt}};

pub fn cbc_encrypt(plaintext: &[u8], key: &[u8; 16], iv: Option<&[u8; 16]>) -> Vec<u8> {
    let mut cipher_text = Vec::new();
    
    // Default IV to all zeros
    let iv = iv.unwrap_or(&[0; 16]);

    // Pad the plaintext to a multiple of the block size
    let padded_plaintext = pkcs7_pad(plaintext, 16);

    let blocks = padded_plaintext.chunks_exact(16).collect::<Vec<&[u8]>>();

    // First block: XOR with IV, then encrypt
    let xor_result = xor_bytes(blocks[0], iv);
    let mut prev_cipher_block = aes_128_ecb_encrypt(&xor_result, key);
    cipher_text.extend_from_slice(&prev_cipher_block);

    // Subsequent blocks: XOR with previous cipher_text block, then encrypt
    for plain_block in blocks.iter().skip(1) {
        let xor_result = xor_bytes(plain_block, &prev_cipher_block);
        prev_cipher_block = aes_128_ecb_encrypt(&xor_result, key);
        cipher_text.extend_from_slice(&prev_cipher_block);
    }

    cipher_text
}

pub fn cbc_decrypt(cipher_text: &[u8], key: &[u8; 16], iv: Option<&[u8; 16]>) -> Vec<u8> {
    let mut plaintext = Vec::new();

    // Default IV to all zeros
    let iv = iv.unwrap_or(&[0; 16]);

    let blocks = cipher_text.chunks_exact(16).collect::<Vec<&[u8]>>();

    let decrypted_block = aes_128_ecb_decrypt(blocks[0], key);
    let xor_result = xor_bytes(&decrypted_block, iv);
    plaintext.extend_from_slice(&xor_result);
    
    let mut prev_cipher_block = blocks[0].to_vec();

    for cipher_block in blocks.iter().skip(1) {
        let decrypted_block = aes_128_ecb_decrypt(cipher_block, key);
        let xor_result = xor_bytes(&decrypted_block, &prev_cipher_block);
        plaintext.extend_from_slice(&xor_result);
        
        prev_cipher_block = cipher_block.to_vec();
    }

    // Remove padding
    pkcs7_unpad(&plaintext)
}