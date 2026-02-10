# SMC Assignments - Garbled Circuits

**Course:** Secure Multi-party Computation (SMC)  
**Date:** January 28, 2026

---

## Assignment 1: Mini Garbled Circuit

### Overview
This assignment introduces Yao's Garbled Circuits through a minimal, intentionally inefficient construction. You will implement a single garbled NAND gate without optimizations or oblivious transfer. The goal is conceptual understanding rather than performance.

### Learning Objectives
After completing this assignment, you should be able to:
- Explain how wire labels represent Boolean values
- Construct and evaluate a garbled gate
- Understand why only one output is revealed

### Threat Model and Scope
- Semi-honest adversaries
- Single Boolean gate (NAND)
- No oblivious transfer
- No networking
- No performance optimization

### Model
There are two roles:
- **Garbler**: constructs the garbled gate
- **Evaluator**: evaluates the garbled gate

Both roles may be implemented within the same program.

### Wire Labels
For each wire w, generate two random labels:
```
L^0_w, L^1_w ∈ {0,1}^k where k ≥ 128
```
These labels encode logical values but do not reveal them.

### Garbling the NAND Gate
For each input combination (a,b) ∈ {0,1}^2:

1. Derive an encryption key:
   ```
   K[a,b] = KDF(L^a_x || L^b_y)
   ```

2. Encrypt the output label:
   ```
   C[a,b] = Enc(K[a,b], L^(a NAND b)_z)
   ```

This results in four ciphertexts:
```
{C[0,0], C[0,1], C[1,0], C[1,1]}
```

The garbled gate is a list of these ciphertexts with random ordering.

### Evaluation
The evaluator:
1. Receives one label for x and one for y
2. Attempts to decrypt all four ciphertexts
3. Exactly one decryption succeeds
4. The decrypted value is the output label

A decoding table maps output labels to Boolean values.

### Implementation Requirements
Your program must:
- Generate wire labels for x, y, and z
- Garble the NAND gate
- Evaluate the garbled gate for given inputs
- Output the correct Boolean result

### Written Explanation (1 page)
Your submission must include a short written explanation addressing:

**(a) Correctness**  
Why does exactly one ciphertext decrypt correctly?

**(b) Inefficiency**  
Explain why four decryptions are required.

### Allowed Tools
You may **NOT** use:
- Existing garbled circuit or MPC frameworks

---

## Assignment 2: Garbled Circuits in Practice

### Overview
Implement a privacy-preserving computation using the NAND gate from Assignment 1. This assignment explores the trade-off between efficiency and leakage.

You will implement the same functionality in two different ways:
- A fully oblivious version (no control-flow leakage)
- A more efficient version that intentionally leaks limited information

You will measure performance and reason about what information is leaked.

### Learning Objectives
After completing this assignment, you should be able to:
- Implement secure two-party computation
- Understand how control flow impacts security and performance
- Distinguish between oblivious and leaky program structures
- Measure and interpret performance costs of secure computation
- Formally describe information leakage

### Problem Description
Two parties, Alice and Bob, each hold a private array of n booleans:
```
A = (a_0, a_1, ..., a_(n-1))
B = (b_0, b_1, ..., b_(n-1))
```

They want to compute if all elements match:
```
∧ a_i = b_i (for all i)
```

### Option A — Fully Oblivious Implementation

Implement the computation in a fully oblivious manner.

**Requirements:**
- All control flow depending on secret data must be oblivious
- No early termination is allowed
- The program must always iterate over all indices

**Expected Behaviour:**
- Runtime is independent of where the first mismatch occurs
- Only the final output is revealed

**Hint:**
You will need to:
- Compute a Boolean flag for each index
- Conjunct all of them

### Option B — Leaky but Efficient Implementation

Implement a second version where performance is improved by allowing controlled leakage.

**Requirements:**
- Use public control flow
- Stop iteration as soon as a mismatch is found
- Reveal the result immediately

**Expected Behaviour:**
- Runtime depends on the location of the first mismatch
- Control flow leaks information about the index

### Performance Evaluation
You must measure and report:
- Runtime for both versions
- Scaling behaviour for at least three input sizes (e.g., n = 4, 8, 16)

### Security Analysis (2 pages)
Your report must include:

**1. Threat Model**  
What information is protected?

**2. Leakage Analysis**
- What is leaked in the oblivious version?
- What additional information is leaked in the early-exit version?

**3. Trade-off Discussion**  
Explain:
- In which real-world scenarios the additional leakage might be acceptable
- When it would be unacceptable

---

## Submission Requirements

Submit:
- Source code
- A 3-page report (PDF)
- Instructions to build and run

**Report Structure:**
- Assignment 1 explanation: 1 page
- Assignment 2 analysis: 2 pages
