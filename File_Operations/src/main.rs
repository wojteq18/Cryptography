mod decrypt_file;
mod encrypt_file;

use cryptography::RSA::{self, generate_rsa_values, generate_e};
use cryptography::unencryption::{self, find_d};
fn main() {
    let (p, q, n, fi_n) = generate_rsa_values();
    let e = generate_e(fi_n);
    let d = find_d(e, fi_n);
    let mut is_ended = false;

    while !is_ended {
        println!("1 -> encrypt file");
        println!("2 -> decrypt file");
        println!("3 -> end");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let action: i8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter 1, 2, or 3.");
                continue;
            }
        };

        match action {
            1 => {
                println!("Enter path of file");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input_file = input.trim().to_string(); // Poprawione

                println!("Enter new path");
                let mut input2 = String::new();
                std::io::stdin().read_line(&mut input2).unwrap();
                let output_file = input2.trim().to_string();

                encrypt_file::encrypt_file(&input_file, &output_file, e, n);
            }
            2 => {
                println!("Enter path of file");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input_file = input.trim().to_string();

                println!("Enter new path");
                let mut input2 = String::new();
                std::io::stdin().read_line(&mut input2).unwrap();
                let output_file = input2.trim().to_string();

                decrypt_file::decrypt_file(&input_file, &output_file, n, d, e);
            }
            3 => {
                is_ended = true;
                println!("Exiting...");
            }
            _ => println!("Invalid action"),
        }
    }
}
