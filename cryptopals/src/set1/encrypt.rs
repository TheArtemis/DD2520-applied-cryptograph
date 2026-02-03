pub fn repeating_key_xor(plain_text: &str, key: &str) -> Vec<u8> {
    
    let mut result = Vec::with_capacity(plain_text.len());
    let plain_text_bytes = plain_text.as_bytes();
    let key_bytes = key.as_bytes();

    for i in 0..plain_text_bytes.len() {
        result.push(plain_text_bytes[i] ^ key_bytes[i % key_bytes.len()]);
    }

    result
}