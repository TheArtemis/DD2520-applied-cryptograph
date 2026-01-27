mod state;
mod alg;
mod gf256;
mod sbox;

use crate::state::State;
use crate::alg::AES128;
use std::fs;

fn main() {
    let input_data = fs::read("data/aes_sample.in")
        .expect("Failed to read input file");
    
    if input_data.len() < 16 {
        eprintln!("Error: Input file must contain at least 16 bytes");
        return;
    }
    
    let mut plaintext = [0u8; 16];
    plaintext.copy_from_slice(&input_data[0..16]);
    
    let key = if input_data.len() >= 32 {
        let mut k = [0u8; 16];
        k.copy_from_slice(&input_data[16..32]);
        k
    } else {
        [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
         0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c]
    };
    
    let mut state = State::new(plaintext);
    
    let aes = AES128::new(key);
    aes.cipher(&mut state);
    
    println!("{}", state);
}
