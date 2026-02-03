use cryptopals::set1::brute_force::decrypt_single_byte_encrypted_hex;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn read_challenge4_input() -> Vec<String> {
    let file = File::open("tests/set1/data/challenge4.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap()).collect()
}

#[test]
fn test_challenge4() {
    let input = read_challenge4_input();
    let mut results: BTreeMap<usize, String> = BTreeMap::new();

    for (i, line) in input.iter().enumerate() {
        let output = decrypt_single_byte_encrypted_hex(line);
        results.insert(i, output);
    }

    let out_path = "tests/set1/data/challenge4_output.txt";
    let mut out_file = File::create(out_path).unwrap();
    for (i, output) in &results {
        writeln!(out_file, "Line {}: {}", i, output).unwrap();
    }
    out_file.flush().unwrap();

    assert_eq!(
        results.get(&170).map(|s| s.trim()),
        Some("Now that the party is jumping"),
        "Line 170 must decrypt to 'Now that the party is jumping'"
    );
}