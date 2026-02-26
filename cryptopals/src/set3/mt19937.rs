// MT19937 Mersenne Twister (32-bit) RNG implementation, based on the reference algorithm.

pub const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u32 = 0x9908_B0DF;
const UPPER_MASK: u32 = 0x8000_0000;
const LOWER_MASK: u32 = 0x7FFF_FFFF;

/// MT19937 PRNG with 32-bit outputs.
pub struct Mt19937 {
    mt: [u32; N],
    index: usize,
}

impl Mt19937 {
    /// Create a new MT19937 instance from a full state array (e.g. after untempering 624 outputs).
    /// Set index to N so the next `next_u32` triggers a twist, then yields the same sequence as the original.
    pub fn from_state(mt: [u32; N]) -> Self {
        Self { mt, index: N }
    }

    /// Create a new MT19937 instance seeded with the given 32-bit value.
    pub fn new(seed: u32) -> Self {
        let mut mt = [0u32; N];
        mt[0] = seed;
        for i in 1..N {
            // f = 1812433253
            let x = mt[i - 1] ^ (mt[i - 1] >> 30);
            mt[i] = 1812433253u32
                .wrapping_mul(x)
                .wrapping_add(i as u32);
        }
        Self { mt, index: N }
    }

    /// Generate the next 32-bit output.
    pub fn next_u32(&mut self) -> u32 {
        if self.index >= N {
            self.twist();
        }

        let mut y = self.mt[self.index];
        self.index += 1;

        // Tempering
        y ^= y >> 11;
        y ^= (y << 7) & 0x9D2C_5680;
        y ^= (y << 15) & 0xEFC6_0000;
        y ^= y >> 18;

        y
    }

    /// Generate the next 32-bit output as an i32 in [0, 0x7FFF_FFFF].
    pub fn next_i31(&mut self) -> i32 {
        (self.next_u32() >> 1) as i32
    }

    fn twist(&mut self) {
        for i in 0..N {
            let x = (self.mt[i] & UPPER_MASK) | (self.mt[(i + 1) % N] & LOWER_MASK);
            let mut x_a = x >> 1;
            if x & 1 != 0 {
                x_a ^= MATRIX_A;
            }
            self.mt[i] = self.mt[(i + M) % N] ^ x_a;
        }
        self.index = 0;
    }
}

