use crate::wire::{WireLabel, WireLabels};
use rand::thread_rng;
use sha2::{Digest, Sha256};

/// Inputs to a garbled NAND gate (labels for x and y).
pub struct GarbledNandInputs {
    pub x: WireLabel,
    pub y: WireLabel,
}

/// Output of evaluating a garbled NAND gate (label for z).
pub struct GarbledNandOutput {
    pub z: WireLabel,
}

/// A single garbled **NAND** gate with fixed input/output labels.
///
/// Logical (non-garbled) NAND gate:
/// - Inputs: bits `x, y` in {0,1}
/// - Output: `z = NAND(x, y) = 1 - (x & y)`
/// - Truth table:
///   - (x, y) = (0, 0) → z = 1
///   - (x, y) = (0, 1) → z = 1
///   - (x, y) = (1, 0) → z = 1
///   - (x, y) = (1, 1) → z = 0
///
/// Garbled representation in this struct:
/// - For each wire `w` in {x, y, z}:
///   - Two random labels L_w^0, L_w^1 encode the bits 0 and 1.
/// - For each input pair (a, b) in {0,1}^2:
///   - Derive a key `K[a,b] = KDF(L_x^a || L_y^b)`.
///   - Encrypt the correct output label `L_z^{NAND(a,b)}` into a ciphertext `C[a,b]`.
/// - The garbled table `table` stores the 4 ciphertexts `{C[0,0], C[0,1], C[1,0], C[1,1]}`
///   in random order.
/// - The evaluator knows one label for `x` and one label for `y` and:
///   - Tries to decrypt all 4 entries.
///   - Exactly one decryption yields a label equal to `L_z^0` or `L_z^1`,
///     which is the (hidden) output bit.
pub struct GarbledNandGate {
    /// Wire labels for inputs x and y and output z.
    pub x_labels: WireLabels,
    pub y_labels: WireLabels,
    pub z_labels: WireLabels,
    /// Garbled table: 4 ciphertexts in random order.
    pub table: Vec<[u8; 32]>,
}

impl GarbledNandGate {
    /// Construct a fresh garbled NAND gate.
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let x_labels = WireLabels::random(&mut rng);
        let y_labels = WireLabels::random(&mut rng);
        let z_labels = WireLabels::random(&mut rng);

        let mut table = Vec::with_capacity(4);
        for (a, b) in &[(0u8, 0u8), (0, 1), (1, 0), (1, 1)] {
            let in_x = if *a == 0 { &x_labels.zero } else { &x_labels.one };
            let in_y = if *b == 0 { &y_labels.zero } else { &y_labels.one };
            let nand_result = if *a == 1 && *b == 1 { 0 } else { 1 };
            let out_z = if nand_result == 0 {
                &z_labels.zero
            } else {
                &z_labels.one
            };

            let k = derive_key(in_x, in_y);
            let c = encrypt_label(&k, out_z);
            table.push(c);
        }

        fastrand::shuffle(&mut table);

        Self {
            x_labels,
            y_labels,
            z_labels,
            table,
        }
    }

    /// Construct a garbled NAND gate with specified wire labels.
    /// Used for circuit construction where wires must share labels across gates.
    pub fn new_with_labels(
        x_labels: WireLabels,
        y_labels: WireLabels,
        z_labels: WireLabels,
    ) -> Self {
        let table = Self::build_table(&x_labels, &y_labels, &z_labels);
        Self {
            x_labels,
            y_labels,
            z_labels,
            table,
        }
    }

    fn build_table(x_labels: &WireLabels, y_labels: &WireLabels, z_labels: &WireLabels) -> Vec<[u8; 32]> {
        let mut table = Vec::with_capacity(4);
        for (a, b) in &[(0u8, 0u8), (0, 1), (1, 0), (1, 1)] {
            let in_x = if *a == 0 { &x_labels.zero } else { &x_labels.one };
            let in_y = if *b == 0 { &y_labels.zero } else { &y_labels.one };
            let nand_result = if *a == 1 && *b == 1 { 0 } else { 1 };
            let out_z = if nand_result == 0 { &z_labels.zero } else { &z_labels.one };
            let k = derive_key(in_x, in_y);
            let c = encrypt_label(&k, out_z);
            table.push(c);
        }
        fastrand::shuffle(&mut table);
        table
    }

    /// Evaluate the garbled NAND gate on given input labels.
    ///
    /// Tries to decrypt all 4 ciphertexts; exactly one will match a known
    /// output label.
    pub fn evaluate(&self, inputs: GarbledNandInputs) -> GarbledNandOutput {
        let mut candidate: Option<WireLabel> = None;

        for c in &self.table {
            let k = derive_key(&inputs.x, &inputs.y);
            let decoded = decrypt_label(&k, c);

            if decoded == self.z_labels.zero || decoded == self.z_labels.one {
                candidate = Some(decoded);
                break;
            }
        }

        GarbledNandOutput {
            z: candidate.expect("exactly one ciphertext should decrypt correctly"),
        }
    }

    /// Decode an output label to its Boolean value.
    ///
    /// This is the "decoding table" that maps output labels to Boolean values.
    /// Returns `Some(bit)` if the label matches one of the known output labels,
    /// or `None` if it's not a valid output label.
    pub fn decode_output(&self, label: &WireLabel) -> Option<bool> {
        if *label == self.z_labels.zero {
            Some(false)
        } else if *label == self.z_labels.one {
            Some(true)
        } else {
            None
        }
    }

    /// Create input labels from Boolean values.
    ///
    /// Helper function for the garbler to encode input bits as labels.
    pub fn encode_inputs(&self, x_bit: bool, y_bit: bool) -> GarbledNandInputs {
        GarbledNandInputs {
            x: if x_bit {
                self.x_labels.one.clone()
            } else {
                self.x_labels.zero.clone()
            },
            y: if y_bit {
                self.y_labels.one.clone()
            } else {
                self.y_labels.zero.clone()
            },
        }
    }
}

fn derive_key(x: &WireLabel, y: &WireLabel) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&x.0);
    hasher.update(&y.0);
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

fn encrypt_label(key: &[u8; 32], label: &WireLabel) -> [u8; 32] {
    // XOR with H(key) as keystream
    let mut hasher = Sha256::new();
    hasher.update(key);
    let keystream = hasher.finalize();

    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        let l = if i < label.0.len() { label.0[i] } else { 0 };
        *b = keystream[i] ^ l;
    }
    out
}

fn decrypt_label(key: &[u8; 32], ciphertext: &[u8; 32]) -> WireLabel {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let keystream = hasher.finalize();

    let mut bytes = [0u8; crate::wire::LABEL_BYTES];
    for i in 0..bytes.len() {
        bytes[i] = keystream[i] ^ ciphertext[i];
    }
    WireLabel(bytes)
}

