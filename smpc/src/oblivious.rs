use crate::composite::garbled_equality;
use crate::composite::garbled_and;
use crate::gate::GarbledNandGate;
use crate::wire::{WireLabel, WireLabels};

/// Fully Oblivious Implementation
/// Computes ∧(a_i == b_i) for all i, always processing all elements
/// Runtime is independent of input values
pub fn array_equality(
    gates: &[GarbledNandGate],
    alice_inputs: &[WireLabel],
    bob_inputs: &[WireLabel],
    _output_labels: &WireLabels,
) -> WireLabel {
    assert_eq!(alice_inputs.len(), bob_inputs.len());
    // Need enough gates: ~8 gates per equality + gates for AND chain
    assert!(gates.len() >= alice_inputs.len() * 10);
    
    let mut gate_idx = 0;
    
    // Compute equality for each index - always process ALL elements
    let mut equality_results = Vec::new();
    for i in 0..alice_inputs.len() {
        let eq = garbled_equality(gates, &mut gate_idx, &alice_inputs[i], &bob_inputs[i]);
        equality_results.push(eq);
    }
    
    // Conjunct all equality results: result[0] AND result[1] AND ... AND result[n-1]
    // Always processes all results, regardless of intermediate values
    let mut result = equality_results[0].clone();
    for i in 1..equality_results.len() {
        result = garbled_and(&gates[gate_idx], &result, &equality_results[i]);
        gate_idx += 1;
    }
    
    result
}
