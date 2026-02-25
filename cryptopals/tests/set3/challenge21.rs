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

