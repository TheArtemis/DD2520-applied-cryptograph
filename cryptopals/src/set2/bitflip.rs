use crate::set2::{oracle::get_random_key, cbc::cbc_encrypt, cbc::cbc_decrypt};

const PREPEND: &str = "comment1=cooking%20MCs;userdata="; // What if we add some malicious data here? like "admin=true" ?
const APPEND: &str = ";comment2=%20like%20a%20pound%20of%20bacon";

pub fn encrypt_user_data(input: &str, key: Option<&[u8; 16]>) -> Vec<u8> {
    // Quote out ";" and "=" characters from user input only
    let sanitized = sanitize_input(input);
    
    // Combine prepend, sanitized user input, and append
    let combined = format!("{}{}{}", PREPEND, sanitized, APPEND);
    
    // Use provided key or generate a random one
    match key {
        Some(k) => cbc_encrypt(combined.as_bytes(), k, None),
        None => {
            let random_key = get_random_key();
            cbc_encrypt(combined.as_bytes(), &random_key, None)
        }
    }
}

pub fn is_admin(ciphertext: &[u8], key: &[u8; 16]) -> bool {
    let decrypted = cbc_decrypt(ciphertext, &key, None, None);
    let decrypted_str = String::from_utf8_lossy(&decrypted);
    decrypted_str.contains("admin=true")
}

fn sanitize_input(input: &str) -> String {
    input.replace(";", "\\;").replace("=", "\\=")
}     

pub fn bitflip_attack(ciphertext: &[u8], injected_bytes: &[u8; 16]) -> Vec<u8> {   

    let mut new_ciphertext = ciphertext.to_vec();
    // Since PREPEND is 32 bytes long, the first block of the ciphertext is the prepended string

    // Right after the prepended string, the user data starts (Block 2)
    //let user_data_block = &ciphertext[32..48];

    // The attack block is the one before the user data block (Block 1)
    let attack_bloc_start_index = 16;

    let known_plaintext = injected_bytes;
    let target_plaintext = b"admin=trueAAAAAA";

    // P[i] = D(C[i]) ^ C[i-1]
    for i in 0..16 {
        let delta = known_plaintext[i] ^ target_plaintext[i];
        new_ciphertext[attack_bloc_start_index + i] ^= delta;        
    }

    new_ciphertext
}