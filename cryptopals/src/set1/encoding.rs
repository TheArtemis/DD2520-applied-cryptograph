pub fn hex_decode(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0, "Hex string must have an even amount of characters");

    // hex representation uses 2 characters per byte
    let mut bytes: Vec<u8> = Vec::with_capacity(s.len() / 2);

    let chars: Vec<char> = s.chars().collect();

    for i in (0..s.len()).step_by(2) {
        let high = hex_value(chars[i]);
        let low = hex_value(chars[i+1]);

        let byte = (high << 4) | low;
        bytes.push(byte);
    }
    
    return bytes;
}

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        let high = hex_char(byte >> 4);
        let low = hex_char(byte & 0x0F);
        hex.push(high);
        hex.push(low);
    }
    hex
}

fn hex_char(byte: u8) -> char {
    match byte {
        0..=9 => (b'0' + byte) as char, // 48u8 
        10..=15 => (b'a' + byte - 10) as char, // 97u8
        _ => panic!("invalid nibble: {}", byte),
    }
}


fn hex_value(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - b'0', // 48u8
        'a'..='f' => c as u8 - b'a' + 10, // 97u8 + 10
        'A'..='F' => c as u8 - b'A' + 10, // 65u8 + 10
        _ => panic!("invalid hex character: {}", c),
    }
}

pub fn base64_encode(bytes: &[u8]) -> String {
    let mut base64 = String::with_capacity(bytes.len() * 2);
    
    for i in (0..bytes.len()).step_by(3) {
        let b0 = bytes[i];
        let b1 = bytes[i+1];
        let b2 = bytes[i+2];

        let combined: u32 = (b0 as u32) << 16 | (b1 as u32) << 8 | (b2 as u32);

        let values = [
            ((combined >> 18) & 0b0011_1111) as u8,
            ((combined >> 12) & 0b0011_1111) as u8,
            ((combined >> 6) & 0b0011_1111) as u8,
            (combined & 0b0011_1111) as u8,
        ];

        for &i in &values {
            base64.push(base64_value(i));
        };
    };

    base64
}

fn base64_value(v: u8) -> char {
    match v {
        0..=25  => (b'A' + v) as char, // 65u8 + 0..=25
        26..=51 => (b'a' + (v - 26)) as char, // 97u8 + 0..=25
        52..=61 => (b'0' + (v - 52)) as char, // 48u8 + 0..=11
        62      => '+', // 43u8
        63      => '/', // 47u8
        _ => unreachable!(),
    }
}

fn base64_char_value(c: char) -> Option<u8> {
    match c {
        'A'..='Z' => Some((c as u8) - b'A'),
        'a'..='z' => Some((c as u8) - b'a' + 26),
        '0'..='9' => Some((c as u8) - b'0' + 52),
        '+' => Some(62),
        '/' => Some(63),
        '=' => None, // padding
        _ => panic!("invalid base64 character: {}", c),
    }
}

pub fn base64_decode(s: &str) -> Vec<u8> {
    let chars: Vec<u8> = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter_map(|c| base64_char_value(c))
        .collect();

    let mut bytes = Vec::with_capacity((chars.len() * 3) / 4);

    for i in (0..chars.len()).step_by(4) {
        let v0 = chars[i];
        let v1 = chars.get(i + 1).copied().unwrap_or(0);
        let v2 = chars.get(i + 2).copied().unwrap_or(0);
        let v3 = chars.get(i + 3).copied().unwrap_or(0);

        bytes.push((v0 << 2) | (v1 >> 4));
        if i + 2 < chars.len() {
            bytes.push((v1 << 4) | (v2 >> 2));
        }
        if i + 3 < chars.len() {
            bytes.push((v2 << 6) | v3);
        }
    }

    bytes
}

pub fn hex_to_base64(hex: &str) -> String {
    let bytes = hex_decode(hex);
    base64_encode(&bytes)
}