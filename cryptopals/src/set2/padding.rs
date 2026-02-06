
pub fn pkcs7_pad_str(data: &str, block_size: usize) -> String {
    let data_bytes = data.as_bytes();
    let padded = pkcs7_pad(data_bytes, block_size);
    String::from_utf8_lossy(&padded).to_string()
}

pub fn pkcs7_unpad(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return data.to_vec();
    }
    let padding_length = data[data.len() - 1] as usize;
    // Only strip if padding looks valid: length in 1..=16 and all pad bytes match
    if padding_length >= 1
        && padding_length <= 16
        && data.len() >= padding_length
        && data[data.len() - padding_length..].iter().all(|&b| b == data[data.len() - 1])
    {
        let mut res = data.to_vec();
        res.truncate(data.len() - padding_length);
        return res;
    }
    data.to_vec()
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

pub fn pkcs7_validate_str(data: &str) -> bool {
    let data_bytes = data.as_bytes();
    pkcs7_validate(data_bytes)
}

pub fn pkcs7_validate(data: &[u8]) -> bool {
    if data.is_empty() {
        return true;
    }
    let padding_length = data[data.len() - 1] as usize;
    padding_length >= 1
        && padding_length <= 16
        && data.len() >= padding_length
        && data[data.len() - padding_length..].iter().all(|&b| b == data[data.len() - 1])
}