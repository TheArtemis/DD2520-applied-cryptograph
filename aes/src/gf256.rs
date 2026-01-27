
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
