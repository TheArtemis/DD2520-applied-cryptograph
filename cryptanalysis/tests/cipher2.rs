use std::fs;
use cryptanalysis::substitution;

#[test]
fn test_break_cipher2() {
    let data_path = if std::path::Path::new("data/cipher2.txt").exists() {
        "data/cipher2.txt"
    } else {
        "../data/cipher2.txt"
    };

    let ciphertext = fs::read_to_string(data_path)
        .expect(&format!("Failed to read cipher2.txt from {}", data_path))
        .trim()
        .to_string();

    println!("\n{}", "=".repeat(80));
    println!("Breaking Substitution Cipher (cipher2.txt)");
    println!("{}", "=".repeat(80));
    println!("\nCiphertext length: {} characters", ciphertext.len());

    // Step 1: Calculate Index of Coincidence
    let ic = substitution::index_of_coincidence(&ciphertext);
    println!("Index of Coincidence: {:.4}", ic);
    if ic > 0.055 {
        println!("  → IC suggests monoalphabetic substitution");
    } else {
        println!("  → IC suggests polyalphabetic or random text");
    }

    // Step 2: Frequency analysis
    println!("\n--- Step 1: Frequency Analysis ---");
    let freq = substitution::frequency_order(&ciphertext);
    println!("Top 10 ciphertext letters:");
    for (ch, count) in freq.iter().take(10) {
        println!("  {}: {}", ch, count);
    }

    // Step 3: Build quadgram model from known English text (cipher1_decrypted)
    let corpus_path = if std::path::Path::new("data/cipher1_decrypted.txt").exists() {
        "data/cipher1_decrypted.txt"
    } else {
        "../data/cipher1_decrypted.txt"
    };
    let corpus = fs::read_to_string(corpus_path)
        .expect(&format!("Failed to read cipher1_decrypted.txt from {}", corpus_path));
    println!("\n--- Step 2: Build English Quadgram Model ---");
    println!(
        "Using corpus from {} ({} chars)",
        corpus_path,
        corpus.len()
    );

    // Step 4: Break the substitution cipher with hillclimbing
    println!("\n--- Step 3: Hillclimbing Search ---");
    let result = substitution::break_substitution(&ciphertext, &corpus);
    assert!(result.is_some(), "Failed to break substitution cipher");

    let result = result.unwrap();
    println!("Recovered key (cipher -> plain):");
    println!("  ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    println!("  {}", result.key_string);
    println!("Score: {:.4}", result.score);

    println!("\n--- Decrypted Text (first 200 chars) ---");
    println!("{}", &result.plaintext[..result.plaintext.len().min(200)]);

    println!("\n--- Decrypted text ---");
    println!("{}", result.plaintext);

    let out_path = if std::path::Path::new("data/cipher2.txt").exists() {
        "data/cipher2_decrypted.txt"
    } else {
        "../data/cipher2_decrypted.txt"
    };
    fs::write(out_path, &result.plaintext).expect("Failed to write cipher2_decrypted.txt");
    println!("\nWritten to {}", out_path);
}
