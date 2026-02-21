use crate::gate::{GarbledNandGate, GarbledNandInputs};
use crate::wire::{WireLabel, WireLabels};
use rand::thread_rng;

/// Build a NOT gate from NAND (NOT(x) = x NAND x)
/// Requires a gate with x_labels = y_labels = input wire labels
fn garbled_not(gate: &GarbledNandGate, input: &WireLabel) -> WireLabel {
    let not_input = GarbledNandInputs {
        x: input.clone(),
        y: input.clone(),
    };
    gate.evaluate(not_input).z
}

/// Build an AND gate from NAND (x AND y = NOT(x NAND y))
/// Requires two gates: one for NAND, one for NOT
pub(crate) fn garbled_and(
    nand_gate: &GarbledNandGate,
    not_gate: &GarbledNandGate,
    x: &WireLabel,
    y: &WireLabel,
) -> WireLabel {
    let nand_result = nand_gate.evaluate(GarbledNandInputs { x: x.clone(), y: y.clone() }).z;
    garbled_not(not_gate, &nand_result)
}

/// Compute equality of two bits: a == b = (a AND b) OR (NOT(a) AND NOT(b))
/// Builds gates with proper wire label connectivity and returns (output_label, output_wire_labels)
pub fn garbled_equality(
    a_labels: &WireLabels,
    b_labels: &WireLabels,
    a: &WireLabel,
    b: &WireLabel,
) -> (WireLabel, WireLabels) {
    let mut rng = thread_rng();

    let not_a_labels = WireLabels::random(&mut rng);
    let not_b_labels = WireLabels::random(&mut rng);
    let nand_ab_labels = WireLabels::random(&mut rng);
    let a_and_b_labels = WireLabels::random(&mut rng);
    let nand_not_a_not_b_labels = WireLabels::random(&mut rng);
    let not_a_and_not_b_labels = WireLabels::random(&mut rng);
    let not_a_and_b_labels = WireLabels::random(&mut rng);
    let not_not_a_and_not_b_labels = WireLabels::random(&mut rng);
    let eq_labels = WireLabels::random(&mut rng);

    // NOT(a) = a NAND a
    let gate0 = GarbledNandGate::new_with_labels(
        a_labels.clone(),
        a_labels.clone(),
        not_a_labels.clone(),
    );
    let not_a = garbled_not(&gate0, a);

    // NOT(b)
    let gate1 = GarbledNandGate::new_with_labels(
        b_labels.clone(),
        b_labels.clone(),
        not_b_labels.clone(),
    );
    let not_b = garbled_not(&gate1, b);

    // a AND b = NOT(NAND(a,b))
    let gate2_nand = GarbledNandGate::new_with_labels(
        a_labels.clone(),
        b_labels.clone(),
        nand_ab_labels.clone(),
    );
    let gate2_not = GarbledNandGate::new_with_labels(
        nand_ab_labels.clone(),
        nand_ab_labels.clone(),
        a_and_b_labels.clone(),
    );
    let nand_ab = gate2_nand.evaluate(GarbledNandInputs { x: a.clone(), y: b.clone() }).z;
    let a_and_b = garbled_not(&gate2_not, &nand_ab);

    // NOT(a) AND NOT(b) = NOT(NAND(not_a, not_b))
    let gate3_nand = GarbledNandGate::new_with_labels(
        not_a_labels.clone(),
        not_b_labels.clone(),
        nand_not_a_not_b_labels.clone(),
    );
    let gate3_not = GarbledNandGate::new_with_labels(
        nand_not_a_not_b_labels.clone(),
        nand_not_a_not_b_labels.clone(),
        not_a_and_not_b_labels.clone(),
    );
    let nand_not_a_not_b = gate3_nand.evaluate(GarbledNandInputs { x: not_a.clone(), y: not_b.clone() }).z;
    let not_a_and_not_b = garbled_not(&gate3_not, &nand_not_a_not_b);

    // NOT(a_and_b), NOT(not_a_and_not_b) for OR input
    let gate4_not_a_and_b = GarbledNandGate::new_with_labels(
        a_and_b_labels.clone(),
        a_and_b_labels.clone(),
        not_a_and_b_labels.clone(),
    );
    let gate4_not_not_a_and_not_b = GarbledNandGate::new_with_labels(
        not_a_and_not_b_labels.clone(),
        not_a_and_not_b_labels.clone(),
        not_not_a_and_not_b_labels.clone(),
    );
    let not_a_and_b = garbled_not(&gate4_not_a_and_b, &a_and_b);
    let not_not_a_and_not_b = garbled_not(&gate4_not_not_a_and_not_b, &not_a_and_not_b);

    // OR = NAND(NOT(x), NOT(y)); a==b = (a AND b) OR (NOT a AND NOT b)
    let gate5 = GarbledNandGate::new_with_labels(
        not_a_and_b_labels.clone(),
        not_not_a_and_not_b_labels.clone(),
        eq_labels.clone(),
    );
    let eq_result = gate5.evaluate(GarbledNandInputs {
        x: not_a_and_b,
        y: not_not_a_and_not_b,
    }).z;

    (eq_result, eq_labels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::GarbledNandGate;
    use rand::thread_rng;

    fn decode(label: &WireLabel, labels: &WireLabels) -> bool {
        *label == labels.one
    }

    #[test]
    fn test_garbled_equality_truth_table() {
        for (a_bit, b_bit) in [(false, false), (false, true), (true, false), (true, true)] {
            let gate = GarbledNandGate::new();
            let a_labels = gate.x_labels.clone();
            let b_labels = gate.y_labels.clone();
            let a = if a_bit { a_labels.one.clone() } else { a_labels.zero.clone() };
            let b = if b_bit { b_labels.one.clone() } else { b_labels.zero.clone() };
            let (eq_result, eq_labels) = garbled_equality(&a_labels, &b_labels, &a, &b);
            let expected = a_bit == b_bit;
            let actual = decode(&eq_result, &eq_labels);
            assert_eq!(actual, expected, "a={} b={}", a_bit, b_bit);
        }
    }
}
