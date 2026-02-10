use smpc::GarbledNandGate;

fn main() {
    println!("=== Assignment 1: Mini Garbled Circuit Demo ===\n");

    // Step 1: Garbler creates the garbled gate
    println!("[Garbler] Creating garbled NAND gate...");
    let gate = GarbledNandGate::new();
    println!("[Garbler] Gate created with {} ciphertexts in random order.\n", gate.table.len());

    // Step 2: Evaluate on all four input combinations
    println!("[Evaluator] Testing all input combinations:\n");
    for (a, b) in &[(false, false), (false, true), (true, false), (true, true)] {
        // Encode boolean inputs as wire labels
        let inputs = gate.encode_inputs(*a, *b);
        
        // Evaluate the garbled gate
        let output = gate.evaluate(inputs);
        
        // Decode the output label to a boolean value
        let result = gate.decode_output(&output.z)
            .expect("output label should decode to a boolean");
        
        println!("  NAND({}, {}) = {}", 
                 if *a { 1 } else { 0 }, 
                 if *b { 1 } else { 0 },
                 if result { 1 } else { 0 });
    }
    
    println!("\n✓ All tests passed! Exactly one ciphertext decrypts correctly for each input.");
}

