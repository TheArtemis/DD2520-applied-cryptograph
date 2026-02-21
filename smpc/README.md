# SMC Assignments — Garbled Circuits

Secure Multi-party Computation assignments: first a minimal garbled NAND gate (Yao-style), then privacy-preserving array equality in both oblivious and leaky flavours.

You’ll need [Rust](https://www.rust-lang.org/) installed (1.70 or newer).

From the `smpc` directory:

```bash
cargo build
```

To run the binary:

```bash
cargo run
```

It mainly points you to the tests. To actually run them:

```bash
cargo test
```

Use `cargo test --test assignment1` for the garbled NAND tests and `cargo test --test assignment2` for the array equality stuff. Add `-- --nocapture` to the assignment2 tests if you want to see the performance numbers printed:

```bash
cargo test --test assignment2 -- --nocapture
```
