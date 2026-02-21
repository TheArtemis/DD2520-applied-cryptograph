use crate::gate::GarbledNandGate;
use crate::oblivious::array_equality as oblivious_array_equality;
use crate::leaky::array_equality as leaky_array_equality;
use crate::wire::{WireLabel, WireLabels};
use rand::thread_rng;
use std::time::Instant;

/// Measure performance of both implementations
pub fn measure_performance(
    n: usize,
    alice_bits: &[bool],
    bob_bits: &[bool],
) -> (f64, f64, usize, usize) {
    assert_eq!(alice_bits.len(), n);
    assert_eq!(bob_bits.len(), n);

    let mut alice_labels = Vec::new();
    let mut bob_labels = Vec::new();
    for _ in 0..n {
        let gate = GarbledNandGate::new();
        alice_labels.push(gate.x_labels);
        bob_labels.push(gate.y_labels);
    }

    let final_output_labels = WireLabels::random(&mut thread_rng());
    let alice_inputs: Vec<WireLabel> = alice_bits
        .iter()
        .enumerate()
        .map(|(i, &bit)| {
            if bit {
                alice_labels[i].one.clone()
            } else {
                alice_labels[i].zero.clone()
            }
        })
        .collect();

    let bob_inputs: Vec<WireLabel> = bob_bits
        .iter()
        .enumerate()
        .map(|(i, &bit)| {
            if bit {
                bob_labels[i].one.clone()
            } else {
                bob_labels[i].zero.clone()
            }
        })
        .collect();

    let start = Instant::now();
    let _oblivious_result = oblivious_array_equality(
        &alice_labels,
        &bob_labels,
        &alice_inputs,
        &bob_inputs,
        &final_output_labels,
    );
    let oblivious_time = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let (_, leaky_index) = leaky_array_equality(
        &alice_labels,
        &bob_labels,
        &alice_inputs,
        &bob_inputs,
        &final_output_labels,
    );
    let leaky_time = start.elapsed().as_secs_f64();

    (oblivious_time, leaky_time, leaky_index, n)
}
