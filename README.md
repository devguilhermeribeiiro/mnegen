# BIP-39 Mnemonic Generator

This Rust program generates a BIP-39 mnemonic phrase and seed from user-provided entropy and an optional passphrase. It uses the `bip39` crate for mnemonic generation and `hex` crate for encoding/decoding.

## Features
- Accepts 16 bytes of entropy in hexadecimal format.
- Accepts an optional passphrase for seed derivation.
- Outputs a BIP-39 mnemonic phrase in English.
- Outputs the derived BIP-39 seed in hexadecimal format.

## Prerequisites
- Rust (stable, latest version recommended)
- Cargo (Rust's package manager)

## Dependencies
- `bip39`: For generating BIP-39 mnemonic phrases and seeds.
- `hex`: For encoding/decoding hexadecimal strings.

Add these to your `Cargo.toml`:
```toml
[dependencies]
bip39 = "2.0"
hex = "0.4"
```

## Usage
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```
2. Build and run the program:
   ```bash
   cargo run
   ```
3. Input 16 bytes of entropy in hexadecimal format (e.g., `a1b2c3d4e5f67890123456789abcdef0`).
4. Input an optional passphrase (press Enter for none).
5. The program outputs the mnemonic phrase and the derived seed.

## Example
```bash
Enter your entropy (16 random bytes in HEX):
a1b2c3d4e5f67890123456789abcdef0
Enter your passphrase:
mysecretpass
MNEMONIC:
word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12
BIP39 SEED WITH PASSPHRASE:
<hex-encoded-seed>
```

## Notes
- Ensure the input entropy is exactly 16 bytes (32 hex characters).
- The program uses the English wordlist for mnemonic generation.
- The seed is derived using the PBKDF2 function as per BIP-39 standard.
- Handle the mnemonic and seed securely, as they are sensitive cryptographic material.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.