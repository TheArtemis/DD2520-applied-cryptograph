use crate::gate::GarbledNandGate;
use crate::oblivious::array_equality as oblivious_array_equality;
use crate::leaky::{array_equality as leaky_array_equality, decode_label};
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
    
    // Setup: Create gates and encode inputs
    let mut gates = Vec::new();
    let mut alice_labels = Vec::new();
    let mut bob_labels = Vec::new();
    let mut equality_output_labels = Vec::new();
    
    for _ in 0..n {
        gates.push(GarbledNandGate::new());
        alice_labels.push(gates[0].x_labels.clone());
        bob_labels.push(gates[0].y_labels.clone());
        equality_output_labels.push(gates[0].z_labels.clone());
    }
    
    // Need more gates for composite operations
    for _ in 0..(n * 10) {
        gates.push(GarbledNandGate::new());
    }
    
    let final_output_labels = WireLabels::random(&mut thread_rng());
    
    // Encode inputs
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
    
    // Measure oblivious version
    let start = Instant::now();
    let _oblivious_result = oblivious_array_equality(
        &gates,
        &alice_inputs,
        &bob_inputs,
        &final_output_labels,
    );
    let oblivious_time = start.elapsed().as_secs_f64();
    
    // Measure leaky version
    let start = Instant::now();
    let (_, leaky_index) = leaky_array_equality(
        &gates,
        &alice_inputs,
        &bob_inputs,
        &equality_output_labels,
        &final_output_labels,
    );
    let leaky_time = start.elapsed().as_secs_f64();
    
    (oblivious_time, leaky_time, leaky_index, n)
}
