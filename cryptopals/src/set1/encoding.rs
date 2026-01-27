fn hex_to_binary(hex: &str) -> String {
    let mut binary = String::new();
    for c in hex.chars() {
       let byte = match c {
         '0' => "0000",
         '1' => "0001",
         '2' => "0010",
         '3' => "0011",
         '4' => "0100", 
         '5' => "0101",
         '6' => "0110",
         '7' => "0111",
         '8' => "1000",
         '9' => "1001",
         'a' => "1010",
         'b' => "1011",
         'c' => "1100",
         'd' => "1101",
         'e' => "1110",
         'f' => "1111",
         _ => panic!("Invalid hex character: {}", c),
       };
       binary.push_str(byte);
    }
    binary
}

fn binary_to_decimal(binary: &str) -> u8 {
    let mut decimal = 0;
    for (i, c) in binary.chars().rev().enumerate() {
        if c == '1' {
            decimal += 2_u8.pow(i as u32);
        }
    }
    decimal
}

fn binary_to_base64(binary: &str) -> String {
    let mut base64 = String::new();
    for i in (0..binary.len()).step_by(6) {
        let chunk = &binary[i..i+6];
        let decimal = binary_to_decimal(chunk);
        let base64_char = match decimal {
            0..=25 => (b'A' + decimal as u8) as char,
            26..=51 => (b'a' + (decimal - 26) as u8) as char,
            52..=61 => (b'0' + (decimal - 52) as u8) as char,
            62 => '+',
            63 => '/',
            _ => panic!("Invalid binary chunk: {}", chunk),
        };
        base64.push(base64_char);
    }
    base64
}

pub fn hex_to_base64(hex: &str) -> String {
    let binary = hex_to_binary(hex);
    binary_to_base64(&binary)
}