# Cryptography Library

A simple Rust library for basic cryptographic operations – including RSA encryption/decryption, key generation, modular arithmetic, and utility functions for working with text.

## Features

- **RSA Encryption & Decryption**  
  Encrypt plaintext by converting characters to ASCII codes and applying RSA encryption via modular exponentiation. Decrypt ciphertext back into readable text.

- **RSA Key Generation**  
  Generate random prime numbers, compute the RSA modulus *n* and Euler's totient (φ(n)), and derive both the public exponent *e* and private exponent *d*.

- **Modular Arithmetic Utilities**  
  Efficient modular exponentiation using the square-and-multiply method.

- **Additional Utilities**  
  Functions to convert strings to ASCII, compute the greatest common divisor (GCD), and more.

## Installation

To use this library in your project, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
cryptography = { git = "https://github.com/wojteq18/Cryptography.git", branch = "master" }
