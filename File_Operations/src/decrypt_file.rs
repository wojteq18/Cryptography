use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use cryptography::unencryption::{self, decrypt};

fn str_to_vec_u128(input: &str) -> Vec<u128> {
    input
        .split_whitespace() // Dzieli na słowa według białych znaków
        .filter_map(|s| s.parse::<u128>().ok()) // Parsuje na u128 i filtruje błędy
        .collect()
}

pub fn decrypt_file(input_path: &str, output_path: &str, n: u128, d: u128, e: u128) {
    let mut file = File::open(input_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let encrypted_blocks: Vec<&str> = contents.lines().collect();
    let mut decrypted_text = String::new();

    for block in encrypted_blocks {
        let encrypted_numbers = str_to_vec_u128(block);
        let decrypted_block = decrypt(encrypted_numbers, n, e, d);
        decrypted_text.push_str(&decrypted_block);
    }

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)
        .unwrap();

    output_file.write_all(decrypted_text.as_bytes()).unwrap();

    println!("Successfully decrypted {} file", input_path);
}
