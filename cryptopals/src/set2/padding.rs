
pub fn pkcs7_pad_str(data: &str, block_size: usize) -> String {
    let data_bytes = data.as_bytes();
    let padded = pkcs7_pad(data_bytes, block_size);
    String::from_utf8_lossy(&padded).to_string()
}

pub fn pkcs7_unpad(data: &[u8]) -> Vec<u8> {
    let padding_length = data[data.len() - 1] as usize;

    let mut res = data.to_vec().clone();
    res.truncate(data.len() - padding_length);
    res
}

pub fn pkcs7_unpad_str(data: &str) -> String {
    let data_bytes = data.as_bytes();
    let res = pkcs7_unpad(data_bytes);
    String::from_utf8_lossy(&res).to_string()
}

pub fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    // Number of bytes to pad
    let padding_length = block_size - (data.len() % block_size);

    // Pad the data with the padding length
    let mut padded = data.to_vec();
    for _ in 0..padding_length {
        padded.push(padding_length as u8);
    }
    padded
}