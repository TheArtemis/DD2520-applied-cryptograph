use crate::gate::{GarbledNandGate, GarbledNandInputs};
use crate::wire::WireLabel;

/// Build a NOT gate from NAND (NOT(x) = x NAND x)
pub fn garbled_not(gate: &GarbledNandGate, input: &WireLabel) -> WireLabel {
    let not_input = GarbledNandInputs {
        x: input.clone(),
        y: input.clone(),
    };
    gate.evaluate(not_input).z
}

/// Build an AND gate from NAND (x AND y = NOT(x NAND y))
pub fn garbled_and(gate: &GarbledNandGate, x: &WireLabel, y: &WireLabel) -> WireLabel {
    let nand_result = gate.evaluate(GarbledNandInputs { x: x.clone(), y: y.clone() }).z;
    garbled_not(gate, &nand_result)
}

/// Build an OR gate from NAND (x OR y = NOT(x) NAND NOT(y))
pub fn garbled_or(gate: &GarbledNandGate, x: &WireLabel, y: &WireLabel) -> WireLabel {
    let not_x = garbled_not(gate, x);
    let not_y = garbled_not(gate, y);
    gate.evaluate(GarbledNandInputs { x: not_x, y: not_y }).z
}

/// Compute equality of two bits using garbled circuits
/// a == b is computed as: (a AND b) OR (NOT(a) AND NOT(b))
/// Uses gates from the pool sequentially
pub fn garbled_equality(
    gates: &[GarbledNandGate],
    gate_idx: &mut usize,
    a: &WireLabel,
    b: &WireLabel,
) -> WireLabel {
    // NOT(a) = a NAND a (uses gates[*gate_idx])
    let not_a = garbled_not(&gates[*gate_idx], a);
    *gate_idx += 1;
    
    // NOT(b) = b NAND b (uses gates[*gate_idx])
    let not_b = garbled_not(&gates[*gate_idx], b);
    *gate_idx += 1;
    
    // (a AND b) = NOT(a NAND b) (uses gates[*gate_idx])
    let a_and_b = garbled_and(&gates[*gate_idx], a, b);
    *gate_idx += 1;
    
    // (NOT(a) AND NOT(b)) (uses gates[*gate_idx])
    let not_a_and_not_b = garbled_and(&gates[*gate_idx], &not_a, &not_b);
    *gate_idx += 1;
    
    // (a AND b) OR (NOT(a) AND NOT(b)) = NOT(NOT(a AND b) NAND NOT(NOT(a) AND NOT(b)))
    let not_a_and_b = garbled_not(&gates[*gate_idx], &a_and_b);
    *gate_idx += 1;
    let not_not_a_and_not_b = garbled_not(&gates[*gate_idx], &not_a_and_not_b);
    *gate_idx += 1;
    
    let or_result = gates[*gate_idx].evaluate(GarbledNandInputs {
        x: not_a_and_b,
        y: not_not_a_and_not_b,
    }).z;
    *gate_idx += 1;
    
    garbled_not(&gates[*gate_idx], &or_result)
}
