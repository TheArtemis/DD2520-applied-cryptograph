# Smart Contracts with Cryptographic Verification

## 1. What is it?

Smart contracts are self-executing digital agreements written as code and stored on a blockchain (like Ethereum). They use cryptographic signatures and verification mechanisms to automatically execute agreed-upon conditions when certain predetermined events occur, without requiring intermediaries or manual approval.

## 2. What is it used for?

Smart contracts are used across many applications:
- **Decentralized Finance (DeFi)**: Lending and trading without central exchanges
- **Supply Chain Management**: Verifying product authenticity and automating payments
- **Real Estate**: Tokenized ownership and automated escrow services
- **Secure Voting**: Tamper-proof and transparent election systems

They eliminate the need for trusted middlemen and ensure agreements execute exactly as coded.

## 3. How it works (roughly)

When a user initiates a smart contract transaction, they sign it with their private key using cryptography (typically ECDSA—Elliptic Curve Digital Signature Algorithm). Network nodes receive the transaction, use the sender's public key to verify the signature is authentic and the data hasn't been altered, and then execute the contract code if all conditions are met. Once validated, the transaction is recorded in a block and cryptographically linked to previous blocks with a hash—if anyone tries to tamper with it, the hash changes, making tampering immediately detectable.
