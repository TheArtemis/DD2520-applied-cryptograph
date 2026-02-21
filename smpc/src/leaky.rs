use crate::composite::garbled_equality;
use crate::wire::{WireLabel, WireLabels};

/// Decode a wire label to a boolean value
pub fn decode_label(label: &WireLabel, labels: &WireLabels) -> bool {
    *label == labels.one
}

/// Leaky but Efficient Implementation
/// Uses public control flow and stops as soon as a mismatch is found
/// Runtime leaks information about where the first mismatch occurs
pub fn array_equality(
    alice_labels: &[WireLabels],
    bob_labels: &[WireLabels],
    alice_inputs: &[WireLabel],
    bob_inputs: &[WireLabel],
    final_output_labels: &WireLabels,
) -> (WireLabel, usize) {
    assert_eq!(alice_inputs.len(), bob_inputs.len());
    assert_eq!(alice_inputs.len(), alice_labels.len());
    assert_eq!(alice_inputs.len(), bob_labels.len());

    for i in 0..alice_inputs.len() {
        let (eq, eq_labels) = garbled_equality(
            &alice_labels[i],
            &bob_labels[i],
            &alice_inputs[i],
            &bob_inputs[i],
        );

        let is_equal = decode_label(&eq, &eq_labels);
        if !is_equal {
            return (final_output_labels.zero.clone(), i);
        }
    }
    (final_output_labels.one.clone(), alice_inputs.len())
}
