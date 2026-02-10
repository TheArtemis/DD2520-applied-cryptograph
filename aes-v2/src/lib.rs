use std::fmt;
use std::ops::{BitXor, BitXorAssign, Index, IndexMut};

// ============================================================
// State representation
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    /* 16 bytes of data
    
        Column Major matrix representation:
    
        | b0  b4  b8  b12 |
        | b1  b5  b9  b13 |
        | b2  b6  b10 b14 |
        | b3  b7  b11 b15 |
    */
    data: [u8; 16],
}

impl State {
    pub fn new(data: [u8; 16]) -> Self {
        Self { data }
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.data
    }

    pub fn zero() -> Self {
        Self::new([0; 16])
    }

    #[inline]
    fn index_offset(row: usize, col: usize) -> usize {
        col * 4 + row
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> u8 {
        assert!(row < 4 && col < 4, "Row and column must be between 0 and 3");
        self.data[Self::index_offset(row, col)]
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        assert!(row < 4 && col < 4, "Row and column must be between 0 and 3");
        self.data[Self::index_offset(row, col)] = value;
    }

    #[inline]
    pub fn get_col(&self, col: usize) -> [u8; 4] {
        assert!(col < 4, "Column must be between 0 and 3");
        [
            self.get(0, col),
            self.get(1, col),
            self.get(2, col),
            self.get(3, col),
        ]
    }

    #[inline]
    pub fn set_col(&mut self, col: usize, values: [u8; 4]) {
        assert!(col < 4, "Column must be between 0 and 3");
        for row in 0..4 {
            self.set(row, col, values[row]);
        }
    }
}

impl Index<(usize, usize)> for State {
    type Output = u8;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(row < 4 && col < 4, "Row and column must be between 0 and 3");
        &self.data[Self::index_offset(row, col)]
    }
}

impl IndexMut<(usize, usize)> for State {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        assert!(row < 4 && col < 4, "Row and column must be between 0 and 3");
        &mut self.data[Self::index_offset(row, col)]
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..4 {
            for col in 0..4 {
                if col > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.get(row, col))?;
            }
            if row < 3 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl BitXor for State {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        let mut result = Self::zero();
        for row in 0..4 {
            for col in 0..4 {
                result[(row, col)] = self[(row, col)] ^ other[(row, col)];
            }
        }
        result
    }
}

impl BitXorAssign for State {
    fn bitxor_assign(&mut self, other: Self) {
        for row in 0..4 {
            for col in 0..4 {
                self[(row, col)] ^= other[(row, col)];
            }
        }
    }
}

// ============================================================
// GF(2^8) helpers
// ============================================================

#[inline(always)]
pub fn gf256_mul(a: u8, b: u8) -> u8 {
    // GF(2^8) multiplication
    let mut result = 0u8;
    let mut a = a;
    let mut b = b;

    while b != 0 {
        if b & 1 != 0 {
            result ^= a;
        }

        a = xtime(a);
        b >>= 1;
    }

    result
}

#[inline(always)]
pub fn xtime(x: u8) -> u8 {
    if x & 0x80 != 0 {
        (x << 1) ^ 0x1B // x^8 + x^4 + x^3 + x + 1
    } else {
        x << 1
    }
}

pub fn gf256_mul2(x: u8) -> u8 {
    xtime(x)
}

pub fn gf256_mul3(x: u8) -> u8 {
    x ^ xtime(x) // 3 = 2 + 1
}

// ============================================================
// S-boxes and cache-priming countermeasure
// ============================================================

pub const AES_SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

pub const AES_INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

#[inline(always)]
pub const fn aes_sbox(x: u8) -> u8 {
    AES_SBOX[x as usize]
}

#[inline(always)]
pub const fn aes_inv_sbox(x: u8) -> u8 {
    AES_INV_SBOX[x as usize]
}

/// Prime the cache lines that contain the AES S-boxes.
///
/// This function performs a fixed, key-independent scan over all
/// entries of `AES_SBOX` and `AES_INV_SBOX`. The goal is that,
/// under a cache access-driven attack (e.g. Prime+Probe), the
/// attacker observes that all cache lines backing the S-boxes are
/// touched in a uniform way, rather than only a secret-dependent
/// subset of them.
#[inline(never)]
pub fn prime_sboxes() {
    let mut acc: u8 = 0;

    for &v in AES_SBOX.iter() {
        acc ^= v;
    }

    for &v in AES_INV_SBOX.iter() {
        acc ^= v;
    }

    let _ = std::hint::black_box(acc);
}

// ============================================================
// AES-128 core (encryption/decryption and key schedule)
// ============================================================

pub struct AES128 {
    key: [u8; 16],
}

impl AES128 {
    pub fn new(key: [u8; 16]) -> Self {
        Self { key }
    }

    pub fn cipher(&self, state: &mut State) {
        let key_schedule = self.key_expansion(self.key);

        self.add_round_key(state, &key_schedule[0]);

        for i in 1..10 {
            prime_sboxes();

            self.sub_bytes(state);
            self.shift_rows(state);
            self.mix_columns(state);
            self.add_round_key(state, &key_schedule[i]);
        }

        prime_sboxes();
        self.sub_bytes(state);
        self.shift_rows(state);
        self.add_round_key(state, &key_schedule[10]);
    }

    /// Inverse cipher (decryption). Decrypts the state in place.
    pub fn inv_cipher(&self, state: &mut State) {
        let key_schedule = self.key_expansion(self.key);

        self.add_round_key(state, &key_schedule[10]);

        for i in (1..10).rev() {
            prime_sboxes();

            self.inv_shift_rows(state);
            self.inv_sub_bytes(state);
            self.add_round_key(state, &key_schedule[i]);
            self.inv_mix_columns(state);
        }

        self.inv_shift_rows(state);
        prime_sboxes();
        self.inv_sub_bytes(state);
        self.add_round_key(state, &key_schedule[0]);
    }

    fn add_round_key(&self, state: &mut State, round_key: &State) {
        *state ^= *round_key;
    }

    fn sub_bytes(&self, state: &mut State) {
        for row in 0..4 {
            for col in 0..4 {
                state[(row, col)] = AES_SBOX[state[(row, col)] as usize];
            }
        }
    }

    fn inv_sub_bytes(&self, state: &mut State) {
        for row in 0..4 {
            for col in 0..4 {
                state[(row, col)] = AES_INV_SBOX[state[(row, col)] as usize];
            }
        }
    }

    fn shift_rows(&self, state: &mut State) {
        // row 0: no shift
        // row 1: left shift by 1
        let temp = state[(1, 0)];
        state[(1, 0)] = state[(1, 1)];
        state[(1, 1)] = state[(1, 2)];
        state[(1, 2)] = state[(1, 3)];
        state[(1, 3)] = temp;

        // row 2: left shift by 2
        let temp0 = state[(2, 0)];
        let temp1 = state[(2, 1)];
        state[(2, 0)] = state[(2, 2)];
        state[(2, 1)] = state[(2, 3)];
        state[(2, 2)] = temp0;
        state[(2, 3)] = temp1;

        // row 3: left shift by 3 (or right shift by 1)
        let temp = state[(3, 3)];
        state[(3, 3)] = state[(3, 2)];
        state[(3, 2)] = state[(3, 1)];
        state[(3, 1)] = state[(3, 0)];
        state[(3, 0)] = temp;
    }

    fn inv_shift_rows(&self, state: &mut State) {
        // row 0: no shift
        // row 1: right shift by 1 (inverse of left by 1)
        let temp = state[(1, 3)];
        state[(1, 3)] = state[(1, 2)];
        state[(1, 2)] = state[(1, 1)];
        state[(1, 1)] = state[(1, 0)];
        state[(1, 0)] = temp;

        // row 2: right shift by 2 (same as left by 2, symmetric)
        let temp0 = state[(2, 0)];
        let temp1 = state[(2, 1)];
        state[(2, 0)] = state[(2, 2)];
        state[(2, 1)] = state[(2, 3)];
        state[(2, 2)] = temp0;
        state[(2, 3)] = temp1;

        // row 3: right shift by 3 (inverse of left by 3 = left by 1)
        let temp = state[(3, 0)];
        state[(3, 0)] = state[(3, 1)];
        state[(3, 1)] = state[(3, 2)];
        state[(3, 2)] = state[(3, 3)];
        state[(3, 3)] = temp;
    }

    fn mix_columns(&self, state: &mut State) {
        for col in 0..4 {
            let mut column = state.get_col(col);
            self.mix_column(&mut column);
            state.set_col(col, column);
        }
    }

    fn mix_column(&self, vec: &mut [u8]) {
        let [a, b, c, d] = [vec[0], vec[1], vec[2], vec[3]];

        vec[0] = gf256_mul2(a) ^ gf256_mul3(b) ^ c ^ d;
        vec[1] = a ^ gf256_mul2(b) ^ gf256_mul3(c) ^ d;
        vec[2] = a ^ b ^ gf256_mul2(c) ^ gf256_mul3(d);
        vec[3] = gf256_mul3(a) ^ b ^ c ^ gf256_mul2(d);
    }

    fn inv_mix_columns(&self, state: &mut State) {
        for col in 0..4 {
            let mut column = state.get_col(col);
            self.inv_mix_column(&mut column);
            state.set_col(col, column);
        }
    }

    fn inv_mix_column(&self, vec: &mut [u8]) {
        let [a, b, c, d] = [vec[0], vec[1], vec[2], vec[3]];

        vec[0] = gf256_mul(a, 0x0e) ^ gf256_mul(b, 0x0b) ^ gf256_mul(c, 0x0d) ^ gf256_mul(d, 0x09);
        vec[1] = gf256_mul(a, 0x09) ^ gf256_mul(b, 0x0e) ^ gf256_mul(c, 0x0b) ^ gf256_mul(d, 0x0d);
        vec[2] = gf256_mul(a, 0x0d) ^ gf256_mul(b, 0x09) ^ gf256_mul(c, 0x0e) ^ gf256_mul(d, 0x0b);
        vec[3] = gf256_mul(a, 0x0b) ^ gf256_mul(b, 0x0d) ^ gf256_mul(c, 0x09) ^ gf256_mul(d, 0x0e);
    }

    fn key_expansion(&self, key: [u8; 16]) -> Vec<State> {
        // split key into 4 words (w[0..3])
        let mut words: [[u8; 4]; 44] = [[0; 4]; 44];

        // initialize first 4 words from key
        for i in 0..4 {
            words[i] = [key[i * 4], key[i * 4 + 1], key[i * 4 + 2], key[i * 4 + 3]];
        }

        // generate remaining 40 words (w[4..43])
        for i in 4..44 {
            let mut temp = words[i - 1];

            if i % 4 == 0 {
                prime_sboxes();

                temp = self.rot_word(temp);
                temp = self.sub_word(temp);
                let rcon = self.rcon(i / 4);
                for j in 0..4 {
                    temp[j] ^= rcon[j];
                }
            }

            // xor with word 4 positions back
            for j in 0..4 {
                words[i][j] = words[i - 4][j] ^ temp[j];
            }
        }

        // group every 4 words into a State (11 round keys total)
        let mut keys = Vec::new();
        for i in 0..11 {
            let mut state_data = [0u8; 16];
            for j in 0..4 {
                let word = words[i * 4 + j];
                state_data[j * 4] = word[0];
                state_data[j * 4 + 1] = word[1];
                state_data[j * 4 + 2] = word[2];
                state_data[j * 4 + 3] = word[3];
            }
            keys.push(State::new(state_data));
        }

        keys
    }

    fn rot_word(&self, word: [u8; 4]) -> [u8; 4] {
        let [a, b, c, d] = word;
        [b, c, d, a]
    }

    fn sub_word(&self, word: [u8; 4]) -> [u8; 4] {
        let [a, b, c, d] = word;
        [
            AES_SBOX[a as usize],
            AES_SBOX[b as usize],
            AES_SBOX[c as usize],
            AES_SBOX[d as usize],
        ]
    }

    fn rcon(&self, round: usize) -> [u8; 4] {
        const RCON_TABLE: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

        if round == 0 {
            [0x00, 0x00, 0x00, 0x00]
        } else if round <= 10 {
            [RCON_TABLE[round - 1], 0x00, 0x00, 0x00]
        } else {
            // for round > 10, compute using xtime (2^(round-1))
            let mut rcon_val = 0x01u8;
            for _ in 1..round {
                rcon_val = xtime(rcon_val);
            }
            [rcon_val, 0x00, 0x00, 0x00]
        }
    }
}

// ============================================================
// ECB helpers
// ============================================================

/// Decrypt ciphertext with AES-128 in ECB mode.
/// `ciphertext.len()` must be a multiple of 16.
pub fn aes_128_ecb_decrypt(ciphertext: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let aes = AES128::new(*key);
    let mut plaintext = Vec::with_capacity(ciphertext.len());
    for block in ciphertext.chunks_exact(16) {
        let mut state = State::new(block.try_into().unwrap());
        aes.inv_cipher(&mut state);
        plaintext.extend_from_slice(state.as_bytes());
    }
    plaintext
}

/// Encrypt plaintext with AES-128 in ECB mode.
/// `plaintext.len()` must be a multiple of 16.
pub fn aes_128_ecb_encrypt(plaintext: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let aes = AES128::new(*key);
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    for block in plaintext.chunks_exact(16) {
        let mut state = State::new(block.try_into().unwrap());
        aes.cipher(&mut state);
        ciphertext.extend_from_slice(state.as_bytes());
    }
    ciphertext
}
