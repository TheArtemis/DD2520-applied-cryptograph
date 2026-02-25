use cryptopals::{
    set1::encoding::base64_decode,
    set3::{
        ctr::{ctr_encrypt, NONCE_ZERO},
        stream_attack::{
            decrypt_with_keystream,
            recover_keystream_repeating_key_style,
            truncate_to_min_length,
        },
    },
};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[test]
fn test_ctr_fixed_nonce_attack_repeating_key_xor_style() {
    let data = include_str!("data/challenge20.txt");
    let lines: Vec<&str> = data.lines().map(str::trim).filter(|s| !s.is_empty()).collect();
    assert_eq!(lines.len(), 60, "expected 60 lines");

    let plaintexts: Vec<Vec<u8>> = lines.iter().map(|s| base64_decode(s)).collect();
    let max_len = plaintexts.iter().map(|p| p.len()).max().unwrap();
    println!(
        "[ch20] Loaded {} plaintexts (max len = {})",
        plaintexts.len(),
        max_len
    );
    for (i, pt) in plaintexts.iter().take(3).enumerate() {
        println!("[ch20] Plaintext {}: {}", i, String::from_utf8_lossy(pt));
    }

    // Fixed nonce 0; random key (fixed seed for reproducible test)
    let mut rng = StdRng::seed_from_u64(20);
    let key: [u8; 16] = rng.gen();

    // Encrypt each line independently under fixed-nonce CTR
    let ciphertexts: Vec<Vec<u8>> = plaintexts
        .iter()
        .map(|pt| ctr_encrypt(pt, &key, &NONCE_ZERO))
        .collect();
    let ct_lengths: Vec<usize> = ciphertexts.iter().map(|c| c.len()).collect();
    println!("[ch20] Ciphertext lengths: {:?}", ct_lengths);

    let (truncated, min_len) = truncate_to_min_length(&ciphertexts);
    println!("[ch20] Using truncated length (keysize) = {}", min_len);

    let concatenated: Vec<u8> = truncated.iter().flatten().copied().collect();
    let keystream = recover_keystream_repeating_key_style(&concatenated, min_len);

    let preview_len = keystream.len().min(32);
    println!(
        "[ch20] Recovered keystream (first {} bytes): {:02x?}",
        preview_len,
        &keystream[..preview_len]
    );

    let recovered_truncated = decrypt_with_keystream(&truncated, &keystream);

    for (i, (orig, rec)) in plaintexts.iter().zip(recovered_truncated.iter()).enumerate() {
        let orig_trunc = &orig[..min_len];
        assert_eq!(orig_trunc.len(), rec.len(), "line {} length mismatch", i);
        let errors = orig_trunc.iter().zip(rec.iter()).filter(|(a, b)| a != b).count();
        println!(
            "[ch20] Line {}: {} byte errors; recovered (truncated): {}",
            i,
            errors,
            String::from_utf8_lossy(rec)
        );
        assert!(
            errors <= 5,
            "line {}: {} byte errors (orig {:?}, rec {:?})",
            i,
            errors,
            String::from_utf8_lossy(orig_trunc),
            String::from_utf8_lossy(rec)
        );
    }
}

