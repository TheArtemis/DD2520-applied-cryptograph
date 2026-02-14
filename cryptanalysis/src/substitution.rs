const ALPHABET_LEN: usize = 26;
const QUADGRAM_COUNT: usize = ALPHABET_LEN * ALPHABET_LEN * ALPHABET_LEN * ALPHABET_LEN;

#[derive(Clone, Copy)]
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (self.state >> 32) as u32
    }

    fn next_f64(&mut self) -> f64 {
        let v = self.next_u32() as f64;
        (v + 1.0) / ((u32::MAX as f64) + 2.0)
    }

    fn gen_range(&mut self, upper: usize) -> usize {
        (self.next_u32() as usize) % upper
    }
}

#[derive(Debug, Clone)]
pub struct SubstitutionResult {
    pub key_map: [u8; ALPHABET_LEN],
    pub key_string: String,
    pub plaintext: String,
    pub score: f64,
}

fn clean_to_indices(text: &str) -> Vec<u8> {
    text.bytes()
        .filter(|b| b.is_ascii_alphabetic())
        .map(|b| b.to_ascii_uppercase() - b'A')
        .collect()
}

fn quad_idx(a: u8, b: u8, c: u8, d: u8) -> usize {
    ((((a as usize) * ALPHABET_LEN + b as usize) * ALPHABET_LEN + c as usize) * ALPHABET_LEN)
        + d as usize
}

fn quadgram_scores(corpus_idx: &[u8]) -> Vec<f64> {
    let mut counts = vec![0u32; QUADGRAM_COUNT];
    for i in 0..corpus_idx.len().saturating_sub(3) {
        let idx = quad_idx(
            corpus_idx[i],
            corpus_idx[i + 1],
            corpus_idx[i + 2],
            corpus_idx[i + 3],
        );
        counts[idx] = counts[idx].saturating_add(1);
    }

    let total: u64 = counts.iter().map(|&c| c as u64).sum();
    if total == 0 {
        return vec![0.0; QUADGRAM_COUNT];
    }

    let floor = (0.01 / total as f64).log10();
    let mut scores = vec![floor; QUADGRAM_COUNT];
    for (i, &c) in counts.iter().enumerate() {
        if c > 0 {
            scores[i] = (c as f64 / total as f64).log10();
        }
    }
    scores
}

fn score_with_key(data_idx: &[u8], key_map: &[u8; ALPHABET_LEN], quad_scores: &[f64]) -> f64 {
    let mut score = 0.0;
    for i in 0..data_idx.len().saturating_sub(3) {
        let a = key_map[data_idx[i] as usize];
        let b = key_map[data_idx[i + 1] as usize];
        let c = key_map[data_idx[i + 2] as usize];
        let d = key_map[data_idx[i + 3] as usize];
        score += quad_scores[quad_idx(a, b, c, d)];
    }
    score
}

fn random_key(rng: &mut Lcg) -> [u8; ALPHABET_LEN] {
    let mut key = [0u8; ALPHABET_LEN];
    for i in 0..ALPHABET_LEN {
        key[i] = i as u8;
    }
    for i in (1..ALPHABET_LEN).rev() {
        let j = rng.gen_range(i + 1);
        key.swap(i, j);
    }
    key
}

fn key_from_frequency(cipher_idx: &[u8], corpus_idx: &[u8]) -> [u8; ALPHABET_LEN] {
    let mut cipher_counts = [0usize; ALPHABET_LEN];
    let mut corpus_counts = [0usize; ALPHABET_LEN];

    for &c in cipher_idx {
        cipher_counts[c as usize] += 1;
    }
    for &c in corpus_idx {
        corpus_counts[c as usize] += 1;
    }

    let mut cipher_order: Vec<usize> = (0..ALPHABET_LEN).collect();
    cipher_order.sort_by(|&a, &b| cipher_counts[b].cmp(&cipher_counts[a]));

    let mut corpus_order: Vec<usize> = (0..ALPHABET_LEN).collect();
    corpus_order.sort_by(|&a, &b| corpus_counts[b].cmp(&corpus_counts[a]));

    let mut key = [255u8; ALPHABET_LEN];
    let mut used = [false; ALPHABET_LEN];

    for (&c, &p) in cipher_order.iter().zip(corpus_order.iter()) {
        key[c] = p as u8;
        used[p] = true;
    }

    let mut remaining = Vec::new();
    for i in 0..ALPHABET_LEN {
        if !used[i] {
            remaining.push(i as u8);
        }
    }

    let mut r_idx = 0usize;
    for i in 0..ALPHABET_LEN {
        if key[i] == 255 {
            key[i] = remaining[r_idx];
            r_idx += 1;
        }
    }

    key
}

fn hillclimb(
    data_idx: &[u8],
    key: &mut [u8; ALPHABET_LEN],
    quad_scores: &[f64],
    rng: &mut Lcg,
    iterations: usize,
    start_temp: f64,
    cooling: f64,
    min_temp: f64,
) -> f64 {
    let mut cur_key = *key;
    let mut cur_score = score_with_key(data_idx, &cur_key, quad_scores);
    let mut best_key = cur_key;
    let mut best_score = cur_score;
    let mut temp = start_temp;

    for _ in 0..iterations {
        let a = rng.gen_range(ALPHABET_LEN);
        let mut b = rng.gen_range(ALPHABET_LEN);
        if a == b {
            b = (b + 1) % ALPHABET_LEN;
        }

        cur_key.swap(a, b);
        let cand_score = score_with_key(data_idx, &cur_key, quad_scores);
        let delta = cand_score - cur_score;
        let accept = delta >= 0.0 || rng.next_f64().ln() < delta / temp;

        if accept {
            cur_score = cand_score;
            if cand_score > best_score {
                best_score = cand_score;
                best_key = cur_key;
            }
        } else {
            cur_key.swap(a, b);
        }

        temp = (temp * cooling).max(min_temp);
    }

    *key = best_key;
    best_score
}

pub fn decrypt_with_key(ciphertext: &str, key_map: &[u8; ALPHABET_LEN]) -> String {
    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let idx = c.to_ascii_uppercase() as u8 - b'A';
                (key_map[idx as usize] + b'A') as char
            } else {
                c
            }
        })
        .collect()
}

pub fn key_as_string(key_map: &[u8; ALPHABET_LEN]) -> String {
    key_map.iter().map(|&v| (v + b'A') as char).collect()
}

pub fn index_of_coincidence(text: &str) -> f64 {
    let mut counts = [0usize; ALPHABET_LEN];
    let mut total = 0usize;

    for b in text.bytes() {
        if b.is_ascii_alphabetic() {
            counts[(b.to_ascii_uppercase() - b'A') as usize] += 1;
            total += 1;
        }
    }

    if total < 2 {
        return 0.0;
    }

    let total_f = total as f64;
    let numerator: f64 = counts
        .iter()
        .map(|&c| {
            let c = c as f64;
            c * (c - 1.0)
        })
        .sum();
    numerator / (total_f * (total_f - 1.0))
}

pub fn frequency_order(text: &str) -> Vec<(char, usize)> {
    let mut counts = [0usize; ALPHABET_LEN];
    for b in text.bytes() {
        if b.is_ascii_alphabetic() {
            counts[(b.to_ascii_uppercase() - b'A') as usize] += 1;
        }
    }
    let mut ordered: Vec<(char, usize)> = (0..ALPHABET_LEN)
        .map(|i| ((b'A' + i as u8) as char, counts[i]))
        .collect();
    ordered.sort_by(|a, b| b.1.cmp(&a.1));
    ordered
}

pub fn break_substitution(ciphertext: &str, corpus: &str) -> Option<SubstitutionResult> {
    let cipher_idx = clean_to_indices(ciphertext);
    let corpus_idx = clean_to_indices(corpus);

    if cipher_idx.len() < 4 || corpus_idx.len() < 4 {
        return None;
    }

    let quad_scores = quadgram_scores(&corpus_idx);
    let sample_len = 2500.min(cipher_idx.len());
    let sample_idx = &cipher_idx[..sample_len];

    let mut rng = Lcg::new(0xC0FFEE_u64 ^ (cipher_idx.len() as u64));
    let initial_key = key_from_frequency(&cipher_idx, &corpus_idx);

    let mut best_key = initial_key;
    let mut best_score = f64::NEG_INFINITY;

    let restarts = 6;
    let sample_iters = 8000;
    let refine_iters = 3000;

    for r in 0..restarts {
        let mut key = if r == 0 { initial_key } else { random_key(&mut rng) };
        let score = hillclimb(
            sample_idx,
            &mut key,
            &quad_scores,
            &mut rng,
            sample_iters,
            20.0,
            0.9995,
            0.3,
        );
        if score > best_score {
            best_score = score;
            best_key = key;
        }
    }

    let final_score = hillclimb(
        &cipher_idx,
        &mut best_key,
        &quad_scores,
        &mut rng,
        refine_iters,
        5.0,
        0.9995,
        0.2,
    );

    let plaintext = decrypt_with_key(ciphertext, &best_key);
    let key_string = key_as_string(&best_key);

    Some(SubstitutionResult {
        key_map: best_key,
        key_string,
        plaintext,
        score: final_score,
    })
}
