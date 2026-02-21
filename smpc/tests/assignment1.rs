use smpc::{GarbledNandGate, GarbledNandInputs};

#[test]
fn test_nand_truth_table() {
    let gate = GarbledNandGate::new();
    let test_cases = vec![
        (false, false, true),
        (false, true, true),
        (true, false, true),
        (true, true, false),
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
    let gate = GarbledNandGate::new();
    for (a, b) in &[(false, false), (false, true), (true, false), (true, true)] {
        let inputs = gate.encode_inputs(*a, *b);
        let output = gate.evaluate(inputs);
        let result = gate.decode_output(&output.z);
        assert!(result.is_some(), "Should decode successfully for inputs ({}, {})", 
                if *a { 1 } else { 0 }, 
                if *b { 1 } else { 0 });
    }
}

#[test]
fn test_garbled_table_randomization() {
    let gate1 = GarbledNandGate::new();
    let gate2 = GarbledNandGate::new();
    let mut differs = false;
    for i in 0..gate1.table.len() {
        if gate1.table[i] != gate2.table[i] {
            differs = true;
            break;
        }
    }
    assert!(differs, "Garbled tables should be randomized");
}

#[test]
fn test_wire_labels_uniqueness() {
    let gate = GarbledNandGate::new();
    assert_ne!(gate.x_labels.zero, gate.x_labels.one);
    assert_ne!(gate.y_labels.zero, gate.y_labels.one);
    assert_ne!(gate.z_labels.zero, gate.z_labels.one);
    assert_ne!(gate.x_labels.zero, gate.y_labels.zero);
    assert_ne!(gate.x_labels.zero, gate.z_labels.zero);
}
