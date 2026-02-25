use cryptopals::set3::mt19937::Mt19937;

#[test]
fn test_mt19937_basic_properties() {
    let mut rng1 = Mt19937::new(5489);
    let mut rng2 = Mt19937::new(5489);
    let mut rng3 = Mt19937::new(1);

    // Same seed -> same sequence
    for _ in 0..10 {
        assert_eq!(rng1.next_u32(), rng2.next_u32());
    }

    // Different seed -> almost certainly different first output
    let a = rng1.next_u32();
    let b = rng3.next_u32();
    assert_ne!(a, b);
}

/// Verify first output for seed 5489 matches reference (MT19937 reference implementation).
#[test]
fn test_mt19937_seed_5489_first_output() {
    let first = Mt19937::new(5489).next_u32();
    assert_eq!(first, 3499211612, "first output for seed 5489 must match reference");
}

