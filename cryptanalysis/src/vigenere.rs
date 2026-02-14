use std::collections::HashMap;

/// Decrypt Vigenère cipher
/// Each character in the ciphertext is shifted backwards by the corresponding key character
pub fn decrypt_vigenere(ciphertext: &str, key: &str) -> String {
    let key_chars: Vec<char> = key.chars().collect();
    let mut key_idx = 0;
    
    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let key_char = key_chars[key_idx % key_chars.len()].to_ascii_uppercase();
                let key_shift = (key_char as u8 - b'A') % 26;
                // Decrypt: subtract the key shift
                let decrypted = ((c as u8 - base + 26 - key_shift) % 26) + base;
                key_idx += 1;
                decrypted as char
            } else {
                c
            }
        })
        .collect()
}

/// Calculate letter frequency distribution
fn letter_frequency(text: &str) -> HashMap<char, f64> {
    let mut counts: HashMap<char, u32> = HashMap::new();
    let mut total = 0u32;
    
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            *counts.entry(ch.to_ascii_uppercase()).or_insert(0) += 1;
            total += 1;
        }
    }
    
    counts.iter()
        .map(|(&ch, &count)| (ch, count as f64 / total as f64))
        .collect()
}

/// Expected English letter frequencies
fn english_frequencies() -> HashMap<char, f64> {
    let mut freq = HashMap::new();
    freq.insert('E', 0.1202);
    freq.insert('T', 0.0910);
    freq.insert('A', 0.0812);
    freq.insert('O', 0.0768);
    freq.insert('I', 0.0731);
    freq.insert('N', 0.0695);
    freq.insert('S', 0.0628);
    freq.insert('H', 0.0602);
    freq.insert('R', 0.0592);
    freq.insert('D', 0.0432);
    freq.insert('L', 0.0398);
    freq.insert('U', 0.0288);
    freq.insert('C', 0.0271);
    freq.insert('M', 0.0261);
    freq.insert('F', 0.0230);
    freq.insert('Y', 0.0211);
    freq.insert('W', 0.0209);
    freq.insert('G', 0.0203);
    freq.insert('P', 0.0182);
    freq.insert('B', 0.0149);
    freq.insert('V', 0.0111);
    freq.insert('K', 0.0069);
    freq.insert('X', 0.0017);
    freq.insert('Q', 0.0011);
    freq.insert('J', 0.0010);
    freq.insert('Z', 0.0007);
    freq
}

/// Calculate Index of Coincidence
/// Values close to 0.065 indicate English text (monoalphabetic)
/// Lower values (0.038-0.045) suggest polyalphabetic cipher
pub fn index_of_coincidence(text: &str) -> f64 {
    let freq = letter_frequency(text);
    let n = text.chars().filter(|c| c.is_ascii_alphabetic()).count() as f64;
    
    if n < 2.0 {
        return 0.0;
    }
    
    freq.values()
        .map(|&p| p * n)
        .map(|count| count * (count - 1.0))
        .sum::<f64>() / (n * (n - 1.0))
}

/// Chi-squared test comparing observed vs expected frequencies
fn chi_squared(observed: &HashMap<char, f64>, expected: &HashMap<char, f64>) -> f64 {
    let mut chi2 = 0.0;
    
    for ch in 'A'..='Z' {
        let obs = observed.get(&ch).copied().unwrap_or(0.0);
        let exp = expected.get(&ch).copied().unwrap_or(0.0);
        
        if exp > 0.0 {
            chi2 += (obs - exp).powi(2) / exp;
        }
    }
    
    chi2
}

/// Find repeated n-grams (sequences of n characters) and their positions
fn find_repeated_ngrams(text: &str, n: usize) -> HashMap<String, Vec<usize>> {
    let mut ngrams: HashMap<String, Vec<usize>> = HashMap::new();
    let chars: Vec<char> = text.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    
    for i in 0..=chars.len().saturating_sub(n) {
        let ngram: String = chars[i..i+n].iter().collect();
        ngrams.entry(ngram).or_insert_with(Vec::new).push(i);
    }
    
    ngrams.retain(|_, positions| positions.len() > 1);
    ngrams
}

/// Kasiski examination: Find likely key length by analyzing distances between repeated patterns
pub fn kasiski_examination(ciphertext: &str) -> Vec<usize> {
    let mut key_lengths = HashMap::new();
    
    // Check n-grams of different lengths (3-5 are most useful)
    for n in 3..=5 {
        let ngrams = find_repeated_ngrams(ciphertext, n);
        
        for (_, positions) in ngrams {
            if positions.len() >= 2 {
                // Calculate distances between occurrences
                for i in 0..positions.len() {
                    for j in (i + 1)..positions.len() {
                        let distance = positions[j] - positions[i];
                        // Find factors of distance (likely key lengths)
                        for factor in 2..=distance.min(30) {
                            if distance % factor == 0 {
                                *key_lengths.entry(factor).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Return most likely key lengths
    let mut lengths: Vec<(usize, usize)> = key_lengths.into_iter().collect();
    lengths.sort_by(|a, b| b.1.cmp(&a.1));
    lengths.into_iter().map(|(len, _)| len).take(10).collect()
}

/// Friedman test: Estimate key length using Index of Coincidence
pub fn friedman_test(ciphertext: &str) -> f64 {
    let n = ciphertext.chars().filter(|c| c.is_ascii_alphabetic()).count() as f64;
    let ic = index_of_coincidence(ciphertext);
    
    // Friedman's formula: m ≈ (0.027 * n) / ((n - 1) * IC - 0.038 * n + 0.065)
    let kappa_p = 0.065; // Expected IC for English
    let kappa_r = 0.038; // Expected IC for random text
    
    if ic <= kappa_r {
        return 1.0;
    }
    
    let numerator = kappa_p - kappa_r;
    let denominator = (n - 1.0) * ic - kappa_r * n + kappa_p;
    
    if denominator.abs() < 0.0001 {
        return 1.0;
    }
    
    (numerator * n) / denominator
}

/// Calculate Index of Coincidence for each column (for Vigenère)
pub fn column_ic(ciphertext: &str, key_length: usize) -> Vec<f64> {
    let chars: Vec<char> = ciphertext.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let mut ics = Vec::new();
    
    for offset in 0..key_length {
        let column: String = chars.iter()
            .enumerate()
            .filter(|(i, _)| i % key_length == offset)
            .map(|(_, &c)| c)
            .collect();
        
        ics.push(index_of_coincidence(&column));
    }
    
    ics
}

/// Find the best Caesar shift for a column using frequency analysis
/// Returns the shift needed to decrypt (which corresponds to the key character)
pub fn find_best_shift_for_column(column: &str) -> u8 {
    let expected_freq = english_frequencies();
    let mut best_shift = 0;
    let mut best_chi2 = f64::MAX;
    
    for shift in 0..26 {
        // Try decrypting this column by shifting BACKWARDS by shift
        // This simulates decrypting with key character = shift
        let decrypted: String = column
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = b'A';
                    // Decrypt: shift backwards (subtract shift)
                    let shifted = ((c as u8 - base + 26 - shift) % 26) + base;
                    shifted as char
                } else {
                    c
                }
            })
            .collect();
        
        let observed_freq = letter_frequency(&decrypted);
        let chi2 = chi_squared(&observed_freq, &expected_freq);
        
        if chi2 < best_chi2 {
            best_chi2 = chi2;
            best_shift = shift;
        }
    }
    
    best_shift
}

/// Attempt to find Vigenère key using frequency analysis on columns
pub fn find_vigenere_key(ciphertext: &str, key_length: usize) -> String {
    let chars: Vec<char> = ciphertext.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let mut key = String::new();
    
    for offset in 0..key_length {
        let column: String = chars.iter()
            .enumerate()
            .filter(|(i, _)| i % key_length == offset)
            .map(|(_, &c)| c)
            .collect();
        
        // Find best decryption shift for this column
        // The decryption shift IS the key character (we decrypt by subtracting the key)
        let decryption_shift = find_best_shift_for_column(&column);
        // The key character is the same as the decryption shift
        key.push((b'A' + decryption_shift) as char);
    }
    
    key
}

/// Break Vigenère cipher: find key length and recover key
pub fn break_vigenere(ciphertext: &str) -> Option<(usize, String, String)> {
    // Step 1: Estimate key length using Friedman test
    let friedman_len = friedman_test(ciphertext).round() as usize;
    
    // Step 2: Use Kasiski examination to get candidate key lengths
    let kasiski_lengths = kasiski_examination(ciphertext);
    
    // Combine and prioritize candidates
    let mut candidates = Vec::new();
    if friedman_len >= 2 && friedman_len <= 30 {
        candidates.push(friedman_len);
    }
    candidates.extend(kasiski_lengths.iter().cloned());
    candidates.sort();
    candidates.dedup();
    
    // Try each candidate key length
    for &key_len in &candidates {
        // Check column ICs - should be close to 0.065 for English
        let column_ics = column_ic(ciphertext, key_len);
        let avg_ic: f64 = column_ics.iter().sum::<f64>() / column_ics.len() as f64;
        
        // If average IC is close to 0.065, this is likely the correct key length
        if avg_ic > 0.055 {
            let key = find_vigenere_key(ciphertext, key_len);
            let decrypted = decrypt_vigenere(ciphertext, &key);
            return Some((key_len, key, decrypted));
        }
    }
    
    // If no good match, try the most likely candidates anyway
    if let Some(&key_len) = candidates.first() {
        let key = find_vigenere_key(ciphertext, key_len);
        let decrypted = decrypt_vigenere(ciphertext, &key);
        return Some((key_len, key, decrypted));
    }
    
    None
}
