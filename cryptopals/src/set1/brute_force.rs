use crate::set1::{distance::hamming_distance, encoding::{base64_decode, hex_decode, hex_encode}};

const COMMON_BYTES: &[u8] = b"etaoin shrdlu";


fn xor_with_single_byte(cipher_text: &[u8], key: u8) -> Vec<u8> {
    let mut result = Vec::with_capacity(cipher_text.len());
    for &byte in cipher_text {
        result.push(byte ^ key);
    }
    result
}

fn xor_with_repeating_key(cipher_text: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        return cipher_text.to_vec();
    }
    cipher_text
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

fn score_bytes(bytes: &[u8]) -> f32 {
    if bytes.is_empty() {
        return 0.0;
    }
    let mut score = 0.0;
    for &byte in bytes {
        if COMMON_BYTES.contains(&byte) {
            score += 1.0;
        }
    }
    score / bytes.len() as f32
}

fn find_best_single_byte_key(cipher_text: &[u8]) -> u8 {
    let mut best_score = 0.0;
    let mut best_key = 0;

    for key in 0..=255 {
        let decrypted = xor_with_single_byte(cipher_text, key);
        let score = score_bytes(&decrypted);
        if score > best_score {
            best_score = score;
            best_key = key;
        };
    };

    best_key
    
}

pub fn decrypt_single_byte_encrypted_hex(cipher_text: &str) -> String {
    let bytes = hex_decode(cipher_text);
    let decrypted = decrypt_single_byte_xor(&bytes);
    String::from_utf8_lossy(&decrypted).to_string()
}

fn decrypt_single_byte_xor(cipher_text: &[u8]) -> Vec<u8> {
    let key = find_best_single_byte_key(cipher_text);
    xor_with_single_byte(cipher_text, key)
}

pub fn find_key_candidates(cipher_text: &[u8], n_candidates: Option<usize>) -> Vec<(usize, f32)> {
    
    let n_candidates = n_candidates.unwrap_or(5);
    
    let mut key_size_scores: Vec<(usize, f32)> = Vec::new();

    for key_size in 2..=40 {
        let score = score_key_size(key_size, cipher_text);
        key_size_scores.push((key_size, score));
    }

    key_size_scores.sort_by(|a, b | a.1.partial_cmp(&b.1).unwrap());

    key_size_scores.into_iter().take(n_candidates).rev().collect()
}

fn score_key_size(key_size: usize, cipher_text: &[u8]) -> f32 {
    // Take the first 4 chunks of the cipher text
    let chunks: Vec<&[u8]> = cipher_text.chunks(key_size).take(4).collect();

    if chunks.len() < 2 {
        return f32::MAX; // Can't compute distance, rank last
    }

    let mut distances = Vec::new();
    for i in 0..chunks.len() - 1 {
        distances.push(hamming_distance(chunks[i], chunks[i + 1]) as f32);
    }

    let avg_distance: f32 = distances.iter().sum::<f32>() / distances.len() as f32;
    avg_distance / key_size as f32
}

pub fn decrypt_repeating_key_xor(cipher_text: &[u8]) -> Vec<u8> {
    if cipher_text.is_empty() {
        return Vec::new();
    }

    // Find the best key size candidates
    let candidates = find_key_candidates(cipher_text, Some(5));

    println!("Candidates: {:?}", candidates);
    
    let mut best_decrypted = Vec::new();
    let mut best_score = 0.0f32;
    let mut best_key = Vec::new();

    for (key_size, _) in candidates.iter() {
        let mut key = Vec::new();
        let chunks: Vec<&[u8]> = cipher_text.chunks(*key_size).collect();

        for key_pos in 0..*key_size {
            let block: Vec<u8> = chunks
                .iter()
                // Filter out chunks that are too short
                .filter_map(|chunk| chunk.get(key_pos).copied()) //
                .collect();

            if block.is_empty() {
                key.push(0);
            } else {
                let key_byte = find_best_single_byte_key(&block);
                key.push(key_byte); 
            }
        }

        let decrypted = xor_with_repeating_key(cipher_text, &key);
        
        // Score the decrypted text
        let score = score_bytes(&decrypted);
        println!("Score: {:.2}, Key: {:?}", score, key);
        if score > best_score {
            best_score = score;
            best_decrypted = decrypted;
            best_key = key.clone();
        }
    }
    println!("--------------------------------");
    println!("Best score: {:.2}, Key: {:?}", best_score, hex_encode(&best_key));
    println!("--------------------------------");
    best_decrypted
}

pub fn decrypt_repeating_key_xor_base64(cipher_text: &str) -> String {
    let bytes = base64_decode(cipher_text);
    let decrypted = decrypt_repeating_key_xor(&bytes);
    String::from_utf8_lossy(&decrypted).to_string()
}









