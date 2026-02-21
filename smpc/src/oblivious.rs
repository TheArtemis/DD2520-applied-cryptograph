use crate::composite::{garbled_and, garbled_equality};
use crate::gate::GarbledNandGate;
use crate::wire::{WireLabel, WireLabels};
use rand::thread_rng;

/// Fully Oblivious Implementation
/// Computes AND_i (a_i == b_i), always processing all elements
/// Runtime is independent of input values
pub fn array_equality(
    alice_labels: &[WireLabels],
    bob_labels: &[WireLabels],
    alice_inputs: &[WireLabel],
    bob_inputs: &[WireLabel],
    final_output_labels: &WireLabels,
) -> WireLabel {
    assert_eq!(alice_inputs.len(), bob_inputs.len());
    assert_eq!(alice_inputs.len(), alice_labels.len());
    assert_eq!(alice_inputs.len(), bob_labels.len());

    let mut equality_results = Vec::new();
    let mut equality_output_labels = Vec::new();
    for i in 0..alice_inputs.len() {
        let (eq, eq_labels) = garbled_equality(
            &alice_labels[i],
            &bob_labels[i],
            &alice_inputs[i],
            &bob_inputs[i],
        );
        equality_results.push(eq);
        equality_output_labels.push(eq_labels);
    }

    if equality_results.len() == 1 {
        let eq_label = &equality_results[0];
        let eq_labels = &equality_output_labels[0];
        if *eq_label == eq_labels.one {
            return final_output_labels.one.clone();
        } else {
            return final_output_labels.zero.clone();
        }
    }

    let mut rng = thread_rng();
    let mut accum_label = equality_results[0].clone();
    let mut accum_labels = equality_output_labels[0].clone();
    for i in 1..equality_results.len() {
        let nand_labels = WireLabels::random(&mut rng);
        let and_labels = if i == equality_results.len() - 1 {
            final_output_labels.clone()
        } else {
            WireLabels::random(&mut rng)
        };
        let nand_gate = GarbledNandGate::new_with_labels(
            accum_labels.clone(),
            equality_output_labels[i].clone(),
            nand_labels.clone(),
        );
        let not_gate = GarbledNandGate::new_with_labels(
            nand_labels.clone(),
            nand_labels.clone(),
            and_labels.clone(),
        );
        accum_label = garbled_and(&nand_gate, &not_gate, &accum_label, &equality_results[i]);
        accum_labels = and_labels;
    }

    accum_label
}
