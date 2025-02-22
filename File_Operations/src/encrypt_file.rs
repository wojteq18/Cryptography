use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use cryptography::unencryption;
use cryptography::encryption::{self, encrypt};
use cryptography::RSA::{self, generate_rsa_values, generate_e};


const BLOCK_SIZE: usize = 4;

pub fn encrypt_file(input_path: &str, output_path: &str, e: u128, n: u128) {
    let mut file = File::open(input_path).unwrap();
    let mut buffer = vec![0; BLOCK_SIZE];
    let mut encrypted_data: Vec<String> = Vec::new();

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        let data_block = &buffer[..bytes_read];

        // Konwersja bajtów na u128 i szyfrowanie
        let data_block_str = std::str::from_utf8(data_block).unwrap();
        let encryption_block = encrypt(&data_block_str, e, n);

        // Zamiana u128 na String
        let encryption_block_str = encryption_block.iter()
            .map(|num| num.to_string()) // Konwersja u128 na String
            .collect::<Vec<String>>()   // Zbiór stringów
            .join(" ");                 // Łączymy spacjami

        encrypted_data.push(encryption_block_str);
    }

    // Zapisywanie zaszyfrowanych liczb jako tekstu
    let mut output_file = OpenOptions::new().write(true).create(true).truncate(true).open(output_path).unwrap();
    for block in encrypted_data.iter() {
        output_file.write_all(block.as_bytes()).unwrap();
        output_file.write_all(b"\n").unwrap();
    }
    println!("Successfully encrypted {}", input_path);
}

