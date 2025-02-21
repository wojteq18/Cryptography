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
        let data_block_str = std::str::from_utf8(&data_block).unwrap();
        let encryption_block = encrypt(&data_block_str, e, n);
        let encryption_block_bytes: Vec<u8> = encryption_block.iter()
            .flat_map(|num| num.to_be_bytes())  // Zamiana u128 na bajty
            .collect();

        let encryption_block_str = encryption_block_bytes.iter()
            .map(|b| format!("{:02X}", b)) // Zamiana bajtów na HEX
            .collect::<Vec<String>>()
            .join(" ");

        encrypted_data.push(encryption_block_str.to_string());
    }


    let mut output_file = OpenOptions::new().write(true).create(true).truncate(true).open(output_path).unwrap();

    for block in encrypted_data.iter() {
        output_file.write_all(block.as_bytes()).unwrap();
        output_file.write_all(b"\n").unwrap();
    }
    println!("Successfully encrypted {}", input_path);
}

pub fn main() {
    let (p, q, n, fi_n) = generate_rsa_values();
    let e = generate_e(fi_n);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input_file = input.trim().to_string();

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).unwrap();
    let mut output_file = input2.trim().to_string();

    encrypt_file(&input_file, &output_file, e, n);

}