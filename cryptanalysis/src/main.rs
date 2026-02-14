use std::fs;
use cryptanalysis::vigenere;

fn main() {
    // Read cipher1.txt
    let ciphertext = fs::read_to_string("data/cipher1.txt")
        .expect("Failed to read cipher1.txt")
        .trim()
        .to_string();
    
    println!("Breaking Vigenère Cipher");
    println!("{}", "=".repeat(80));
    println!("\nCiphertext length: {} characters", ciphertext.len());
    
    // Step 1: Calculate Index of Coincidence
    let ic = vigenere::index_of_coincidence(&ciphertext);
    println!("Index of Coincidence: {:.4}", ic);
    if ic < 0.055 {
        println!("  → Low IC suggests polyalphabetic cipher (Vigenère)");
    }
    
    // Step 2: Friedman test to estimate key length
    println!("\n--- Step 1: Estimate Key Length ---");
    let friedman_len = vigenere::friedman_test(&ciphertext);
    println!("Friedman test estimated key length: {:.1}", friedman_len);
    
    // Step 3: Kasiski examination
    println!("\n--- Step 2: Kasiski Examination ---");
    let kasiski_lengths = vigenere::kasiski_examination(&ciphertext);
    println!("Key length candidates from repeated patterns: {:?}", kasiski_lengths);
    
    // Step 4: Try to break the cipher
    println!("\n--- Step 3: Recover Key Using Frequency Analysis ---");
    if let Some((key_len, recovered_key, decrypted)) = vigenere::break_vigenere(&ciphertext) {
        println!("Recovered key length: {}", key_len);
        println!("Recovered key: {}", recovered_key);
        
        // Check column ICs
        let column_ics = vigenere::column_ic(&ciphertext, key_len);
        println!("\nColumn Index of Coincidence values:");
        for (i, ic) in column_ics.iter().enumerate() {
            println!("  Column {}: {:.4}", i + 1, ic);
        }
        
        println!("\n--- Decrypted Text ---");
        println!("{}", decrypted);
        
        // Show column analysis
        println!("\n--- Column Analysis ---");
        let chars: Vec<char> = ciphertext.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        for offset in 0..key_len {
            let column: String = chars.iter()
                .enumerate()
                .filter(|(i, _)| i % key_len == offset)
                .map(|(_, &c)| c)
                .collect();
            
            println!("\nColumn {} (key position {}):", offset + 1, offset + 1);
            let shift = vigenere::find_best_shift_for_column(&column);
            let key_char = (b'A' + shift) as char;
            println!("  Best shift: {} (key char: {})", shift, key_char);
            println!("  Column length: {} characters", column.len());
        }
    } else {
        println!("Failed to break the cipher automatically.");
    }
}
