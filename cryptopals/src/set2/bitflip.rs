use crate::set2::{oracle::get_random_key, cbc::cbc_encrypt};

const APPEND: &str = ";comment2=%20like%20a%20pound%20of%20bacon";
const PREPEND: &str = "comment1=cooking%20MCs;userdata=";

pub fn first_function(input: &str, key: Option<&[u8; 16]>) -> Vec<u8> {
    // Quote out ";" and "=" characters from user input only
    let quoted_input = input.replace(";", "\\;").replace("=", "\\=");
    
    // Combine prepend, quoted user input, and append
    let combined = format!("{}{}{}", PREPEND, quoted_input, APPEND);
    
    // Use provided key or generate a random one
    match key {
        Some(k) => cbc_encrypt(combined.as_bytes(), k, None),
        None => {
            let random_key = get_random_key();
            cbc_encrypt(combined.as_bytes(), &random_key, None)
        }
    }
}
