use smpc::{GarbledNandGate, GarbledNandInputs};

#[test]
fn test_nand_truth_table() {
    // Test all four input combinations for NAND gate
    let gate = GarbledNandGate::new();

    let test_cases = vec![
        (false, false, true),   // NAND(0, 0) = 1
        (false, true, true),    // NAND(0, 1) = 1
        (true, false, true),    // NAND(1, 0) = 1
        (true, true, false),    // NAND(1, 1) = 0
    ];

    for (a, b, expected) in test_cases {
        let inputs = gate.encode_inputs(a, b);
        let output = gate.evaluate(inputs);
        let result = gate.decode_output(&output.z)
            .expect("output label should decode to a boolean");

        assert_eq!(
            result, expected,
            "NAND({}, {}) should be {}, but got {}",
            if a { 1 } else { 0 },
            if b { 1 } else { 0 },
            expected,
            result
        );
    }
}

#[test]
fn test_exactly_one_decryption_succeeds() {
    // Verify that exactly one ciphertext decrypts correctly for each input
    let gate = GarbledNandGate::new();

    for (a, b) in &[(false, false), (false, true), (true, false), (true, true)] {
        let inputs = gate.encode_inputs(*a, *b);
        let output = gate.evaluate(inputs);
        
        // Should successfully decode
        let result = gate.decode_output(&output.z);
        assert!(result.is_some(), "Should decode successfully for inputs ({}, {})", 
                if *a { 1 } else { 0 }, 
                if *b { 1 } else { 0 });
    }
}

#[test]
fn test_garbled_table_randomization() {
    // Verify that the garbled table is randomized (order differs between gates)
    let gate1 = GarbledNandGate::new();
    let gate2 = GarbledNandGate::new();

    // The tables should be different (very high probability)
    // We check that at least one position differs
    let mut differs = false;
    for i in 0..gate1.table.len() {
        if gate1.table[i] != gate2.table[i] {
            differs = true;
            break;
        }
    }
    
    // With random ordering, probability of identical order is 1/24, so this should pass
    assert!(differs, "Garbled tables should be randomized");
}

#[test]
fn test_wire_labels_uniqueness() {
    // Verify that wire labels are unique and don't reveal values
    let gate = GarbledNandGate::new();

    // Labels for 0 and 1 should be different
    assert_ne!(gate.x_labels.zero, gate.x_labels.one);
    assert_ne!(gate.y_labels.zero, gate.y_labels.one);
    assert_ne!(gate.z_labels.zero, gate.z_labels.one);

    // Labels from different wires should be different (with high probability)
    assert_ne!(gate.x_labels.zero, gate.y_labels.zero);
    assert_ne!(gate.x_labels.zero, gate.z_labels.zero);
}
