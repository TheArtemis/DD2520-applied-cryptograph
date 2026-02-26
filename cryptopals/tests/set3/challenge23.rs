// Challenge 23: Clone MT19937 from 624 outputs by inverting the tempering transform.
//
// Hardening: To make this attack hard, don't expose the raw MT output. For example:
// - Hash each tempered output (e.g. SHA-256) and expose only the hash. Then the attacker
//   cannot invert the temper or recover state; they'd have to invert the hash instead.
// - Use a CSPRNG for security-sensitive use; MT19937 is predictable from 624 outputs.

use cryptopals::set3::{
    mt19937::{Mt19937, N},
    mt19937_attack::{clone_from_outputs, untemper},
};

/// Apply the same tempering as MT19937 (for testing untemper round-trip).
fn temper(y: u32) -> u32 {
    let mut y = y;
    y ^= y >> 11;
    y ^= (y << 7) & 0x9D2C_5680;
    y ^= (y << 15) & 0xEFC6_0000;
    y ^= y >> 18;
    y
}

#[test]
fn test_untemper_round_trip() {
    // Untempering should invert tempering: temper(untemper(out)) == out for any output.
    let mut rng = Mt19937::new(12345);
    for _ in 0..10 {
        let out = rng.next_u32();
        assert_eq!(temper(untemper(out)), out);
    }
}

#[test]
fn test_clone_predicts_original() {
    let mut original = Mt19937::new(5489);

    // Collect 624 outputs (one full state batch).
    let mut outputs = [0u32; N];
    for i in 0..N {
        outputs[i] = original.next_u32();
    }

    let mut cloned = clone_from_outputs(&outputs);

    // The cloned generator should match the original from here on.
    for _ in 0..100 {
        assert_eq!(
            original.next_u32(),
            cloned.next_u32(),
            "cloned MT19937 must predict original after 624 outputs"
        );
    }
}
