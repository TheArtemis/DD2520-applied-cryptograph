use crate::composite::garbled_equality;
use crate::gate::GarbledNandGate;
use crate::wire::{WireLabel, WireLabels};

/// Decode a wire label to a boolean value
pub fn decode_label(label: &WireLabel, labels: &WireLabels) -> bool {
    *label == labels.one
}

/// Leaky but Efficient Implementation
/// Uses public control flow and stops as soon as a mismatch is found
/// Runtime leaks information about where the first mismatch occurs
pub fn array_equality(
    gates: &[GarbledNandGate],
    alice_inputs: &[WireLabel],
    bob_inputs: &[WireLabel],
    equality_output_labels: &[WireLabels],
    final_output_labels: &WireLabels,
) -> (WireLabel, usize) {
    assert_eq!(alice_inputs.len(), bob_inputs.len());
    assert_eq!(alice_inputs.len(), equality_output_labels.len());
    assert!(gates.len() >= alice_inputs.len() * 10);
    
    let mut gate_idx = 0;
    
    // Public control flow: iterate and stop at first mismatch
    // This leaks the index where mismatch occurs!
    for i in 0..alice_inputs.len() {
        let eq = garbled_equality(gates, &mut gate_idx, &alice_inputs[i], &bob_inputs[i]);
        
        // Decode intermediate result (LEAKY - reveals information!)
        let is_equal = decode_label(&eq, &equality_output_labels[i]);
        
        if !is_equal {
            // Found mismatch - stop early (LEAKS INDEX i!)
            return (final_output_labels.zero.clone(), i);
        }
    }
    
    // All were equal
    (final_output_labels.one.clone(), alice_inputs.len())
}
