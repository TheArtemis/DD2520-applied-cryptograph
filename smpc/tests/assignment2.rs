use smpc::{oblivious_array_equality, leaky_array_equality, measure_performance};
use smpc::{GarbledNandGate, WireLabels};
use rand::thread_rng;

#[test]
fn test_oblivious_all_equal() {
    let n = 4;
    let mut alice_labels = Vec::new();
    let mut bob_labels = Vec::new();
    for _ in 0..n {
        let gate = GarbledNandGate::new();
        alice_labels.push(gate.x_labels);
        bob_labels.push(gate.y_labels);
    }

    let final_output_labels = WireLabels::random(&mut thread_rng());
    let alice_inputs: Vec<_> = (0..n)
        .map(|i| alice_labels[i].one.clone())
        .collect();
    let bob_inputs: Vec<_> = (0..n)
        .map(|i| bob_labels[i].one.clone())
        .collect();

    let result = oblivious_array_equality(
        &alice_labels,
        &bob_labels,
        &alice_inputs,
        &bob_inputs,
        &final_output_labels,
    );

    let decoded = result == final_output_labels.one;
    assert!(decoded, "All equal arrays should return true");
}

#[test]
fn test_oblivious_all_different() {
    let n = 4;
    let mut alice_labels = Vec::new();
    let mut bob_labels = Vec::new();
    for _ in 0..n {
        let gate = GarbledNandGate::new();
        alice_labels.push(gate.x_labels);
        bob_labels.push(gate.y_labels);
    }

    let final_output_labels = WireLabels::random(&mut thread_rng());
    let alice_inputs: Vec<_> = (0..n)
        .map(|i| if i % 2 == 0 { alice_labels[i].one.clone() } else { alice_labels[i].zero.clone() })
        .collect();
    let bob_inputs: Vec<_> = (0..n)
        .map(|i| if i % 2 == 0 { bob_labels[i].zero.clone() } else { bob_labels[i].one.clone() })
        .collect();

    let result = oblivious_array_equality(
        &alice_labels,
        &bob_labels,
        &alice_inputs,
        &bob_inputs,
        &final_output_labels,
    );

    let decoded = result == final_output_labels.zero;
    assert!(decoded, "Different arrays should return false");
}

#[test]
fn test_oblivious_all_different_correct() {
    let n = 4;
    let mut alice_labels = Vec::new();
    let mut bob_labels = Vec::new();
    for _ in 0..n {
        let gate = GarbledNandGate::new();
        alice_labels.push(gate.x_labels);
        bob_labels.push(gate.y_labels);
    }

    let final_output_labels = WireLabels::random(&mut thread_rng());
    let alice_inputs: Vec<_> = (0..n)
        .map(|i| if i % 2 == 0 { alice_labels[i].one.clone() } else { alice_labels[i].zero.clone() })
        .collect();
    let bob_inputs: Vec<_> = (0..n)
        .map(|i| if i % 2 == 0 { bob_labels[i].zero.clone() } else { bob_labels[i].one.clone() })
        .collect();

    let result = oblivious_array_equality(
        &alice_labels,
        &bob_labels,
        &alice_inputs,
        &bob_inputs,
        &final_output_labels,
    );

    let decoded = result == final_output_labels.zero;
    assert!(decoded, "Different arrays should return false");
}

#[test]
fn test_leaky_early_termination() {
    let n = 4;
    let mut alice_labels = Vec::new();
    let mut bob_labels = Vec::new();
    for _ in 0..n {
        let gate = GarbledNandGate::new();
        alice_labels.push(gate.x_labels);
        bob_labels.push(gate.y_labels);
    }

    let final_output_labels = WireLabels::random(&mut thread_rng());
    let mut alice_inputs = vec![alice_labels[0].one.clone()];
    let mut bob_inputs = vec![bob_labels[0].zero.clone()];
    for i in 1..n {
        alice_inputs.push(alice_labels[i].one.clone());
        bob_inputs.push(bob_labels[i].one.clone());
    }

    let (result, index) = leaky_array_equality(
        &alice_labels,
        &bob_labels,
        &alice_inputs,
        &bob_inputs,
        &final_output_labels,
    );

    assert_eq!(index, 0, "Should stop at first mismatch (index 0)");
    let decoded = result == final_output_labels.zero;
    assert!(decoded, "Should return false");
}

#[test]
fn test_performance_measurement() {
    let n = 4;
    let alice_bits = vec![true, true, false, false];
    let bob_bits = vec![true, true, false, false];

    let (oblivious_time, leaky_time, leaky_index, measured_n) = measure_performance(
        n,
        &alice_bits,
        &bob_bits,
    );

    assert_eq!(measured_n, n);
    assert!(oblivious_time >= 0.0);
    assert!(leaky_time >= 0.0);
    assert_eq!(leaky_index, n);
}

#[test]
fn test_scaling_behavior() {
    let sizes = vec![4, 8, 16];

    for &n in &sizes {
        let alice_bits: Vec<bool> = (0..n).map(|i| i % 2 == 0).collect();
        let bob_bits: Vec<bool> = (0..n).map(|i| i % 2 == 0).collect();

        let (oblivious_time, leaky_time, _, _) = measure_performance(
            n,
            &alice_bits,
            &bob_bits,
        );

        println!("n={}: oblivious={:.6}s, leaky={:.6}s", n, oblivious_time, leaky_time);

        assert!(oblivious_time >= 0.0);
        assert!(leaky_time >= 0.0);
    }
}
