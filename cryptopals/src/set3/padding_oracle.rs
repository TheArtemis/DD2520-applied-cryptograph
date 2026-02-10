use crate::set2::padding::pkcs7_unpad;
use crate::set3::utils::PaddingOracle;

const BLOCK_SIZE: usize = 16;

/// Recovers the full plaintext using the padding oracle.
///
/// * `ciphertext`: full CBC ciphertext (multiple of 16 bytes).
/// * `iv`: the IV used for encryption (16 bytes).
/// * `oracle`: the padding oracle (holds the key; attacker only gets boolean padding feedback).
pub fn padding_oracle_attack(ciphertext: &[u8], iv: &[u8; 16], oracle: &PaddingOracle) -> Vec<u8> {
    assert_eq!(ciphertext.len() % BLOCK_SIZE, 0);
    let blocks: Vec<&[u8]> = ciphertext.chunks_exact(BLOCK_SIZE).collect();
    let n = blocks.len();
    if n == 0 {
        return vec![];
    }

    let mut plaintext_blocks: Vec<[u8; BLOCK_SIZE]> = Vec::with_capacity(n);

    // Decrypt from last block to first. For each block we only need that block and the
    // "previous" block (or IV for the first ciphertext block). We force valid padding
    // on a constructed ciphertext whose last decrypted block is Dec(C_i) XOR G.
    for block_index in (0..n).rev() {
        let decrypted_block = decrypt_block(block_index, &blocks, iv, oracle);
        plaintext_blocks.push(decrypted_block);
    }

    // We collected [P_n, P_{n-1}, ..., P_1]; concatenate in correct order.
    let mut ordered = Vec::with_capacity(n * BLOCK_SIZE);
    for block in plaintext_blocks.into_iter().rev() {
        ordered.extend_from_slice(&block);
    }

    // Strip PKCS#7 padding from the final message.
    pkcs7_unpad(&ordered)
}

/// Decrypts a single block at `block_index` (0 = first block) using the oracle.
///
/// **Idea:** In CBC, the last plaintext block is P' = Dec(C) XOR prev_block.
/// We control prev_block (call it G). So we set G so that P' has valid padding.
/// When the oracle returns true, we know the last byte(s) of P'; that reveals
/// the corresponding byte(s) of Dec(C), hence of the real plaintext P = Dec(C) XOR C_prev.
fn decrypt_block(
    block_index: usize,
    blocks: &[&[u8]],
    iv: &[u8; 16],
    oracle: &PaddingOracle,
) -> [u8; BLOCK_SIZE] {
    let prev_block: [u8; 16] = if block_index == 0 {
        *iv
    } else {
        blocks[block_index - 1].try_into().unwrap()
    };

    // We will build G (modified previous block) and recover Dec(C) byte by byte.
    // Dec(C) is the raw block-cipher output before XOR with previous block.
    let mut g = [0u8; BLOCK_SIZE];
    let mut dec_c = [0u8; BLOCK_SIZE]; // Dec(current_block), filled from last byte to first

    // Decrypt from last byte (index 15) to first (index 0).
    for pos in (0..BLOCK_SIZE).rev() {
        // --- How padding is forced ---
        // We want the decrypted block P' = Dec(C) XOR G to end with valid PKCS#7:
        // P'[pos..16] = L, L, ..., L with L = 16 - pos (so L in 1..=16).
        // We already recovered dec_c[pos+1..16] in previous iterations. So we set
        // G[j] = dec_c[j] XOR L for j in pos+1..16, giving P'[j] = Dec(C)[j] XOR G[j] = L.
        let padding_len = (BLOCK_SIZE - pos) as u8;

        for j in (pos + 1)..BLOCK_SIZE {
            g[j] = dec_c[j] ^ padding_len;
        }

        // --- Why the oracle response leaks information ---
        // We brute-force G[pos] over 0..=255. The oracle decrypts our (ct, iv), so it sees
        // P' = Dec(C) XOR G. When it returns true, the last byte of the full plaintext is
        // valid padding: P'[pos] = L (almost always L=1 for single-byte padding).
        // So Dec(C)[pos] XOR G[pos] = L  =>  Dec(C)[pos] = G[pos] XOR L.
        // --- Why each byte modification works ---
        // We control G; CBC gives P' = Dec(C) XOR G bytewise. So changing G[pos] only
        // changes P'[pos]. One value makes P'[pos] = L and the rest of P' already has
        // P'[pos+1..] = L, so the oracle sees valid padding and returns true.
        let mut found = false;
        for g_byte in 0u8..=255 {
            g[pos] = g_byte;
            if query_oracle(block_index, blocks, iv, &g, oracle) {
                dec_c[pos] = g_byte ^ padding_len;
                found = true;
                break;
            }
        }
        assert!(
            found,
            "padding oracle attack failed at block {} pos {}",
            block_index, pos
        );
    }

    // Real plaintext: P = Dec(C) XOR prev_block (the actual previous ciphertext block or IV).
    let mut plain = [0u8; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        plain[i] = dec_c[i] ^ prev_block[i];
    }
    plain
}

/// Builds the ciphertext to send to the oracle and calls it.
///
/// For block_index 0: we send ciphertext = [current_block] and iv = G, so the only
/// decrypted block is P' = Dec(C_0) XOR G.
/// For block_index i > 0: we send iv = original iv, ciphertext = C_0 .. C_{i-2} || G || C_i,
/// so the last decrypted block is P' = Dec(C_i) XOR G.
/// The oracle only sees (ciphertext, iv); it uses the real key internally to decrypt and check padding.
fn query_oracle(
    block_index: usize,
    blocks: &[&[u8]],
    iv: &[u8; 16],
    g: &[u8; 16],
    oracle: &PaddingOracle,
) -> bool {
    let mut ct = Vec::with_capacity((block_index + 1) * BLOCK_SIZE);
    if block_index == 0 {
        ct.extend_from_slice(blocks[0]);
        oracle.check_padding(&ct, g)
    } else {
        for j in 0..block_index - 1 {
            ct.extend_from_slice(blocks[j]);
        }
        ct.extend_from_slice(g);
        ct.extend_from_slice(blocks[block_index]);
        oracle.check_padding(&ct, iv)
    }
}
