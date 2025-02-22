# Cryptography Library

A simple Rust library for basic cryptographic operations, including RSA encryption/decryption, key generation, modular arithmetic, and utility functions for working with text.

---

## Requirements

To run this program, you need:

- Rust  
- Cargo  
- Internet connection (to fetch the library from GitHub)  

---

## Features

### RSA Encryption & Decryption
- Encrypts plaintext by converting characters to ASCII and applying RSA encryption using modular exponentiation.
- Decrypts ciphertext back into readable text.

### RSA Key Generation
- Generates random prime numbers, computes the RSA modulus `n` and Euler's totient `φ(n)`.
- Derives both the public exponent `e` and the private exponent `d`.

### Modular Arithmetic Utilities
- Efficient modular exponentiation using the square-and-multiply method.

### Additional Utilities
- Functions to convert strings to ASCII.
- Compute the greatest common divisor (GCD).
- More utility functions for cryptographic operations.

---

## Installation

To use this library in your Rust project, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
cryptography = { git = "https://github.com/wojteq18/Cryptography.git", branch = "master" }
```

Then build your project with:

```sh
cargo build
```

---

## Running the Program

To execute the main program, run:

```sh
cargo run
```

If you want to encrypt text files, navigate to the **File_Operations** directory and run:

```sh
cd File_Operations
cargo run
```

This functionality will be added in future updates.

---

## Future Updates

Planned improvements:

- Support for encrypting and decrypting text files  
- Handling larger RSA keys (e.g., 2048-bit keys)  
- Better optimization of encryption algorithms  

---

## License

This project is licensed under the **MIT License** – feel free to use, modify, and share it.

Have an idea for a new feature? Open a pull request on GitHub.
