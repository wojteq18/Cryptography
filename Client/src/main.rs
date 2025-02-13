use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use std::thread;
use std::sync::{Arc, Mutex};
use cryptography::unencryption;
use cryptography::encryption::{self, encrypt};
use cryptography::RSA;

fn string_to_vec_u128(s: &str) -> Vec<u128> {
    s.chars().map(|c| c as u128).collect()
}

fn main() -> std::io::Result<()> {
    let (p, q, n, fi_n) = RSA::generate_rsa_values();   
    let e = RSA::generate_e(fi_n);
    // Łączenie z serwerem
    let mut stream = TcpStream::connect("127.0.0.1:9999")?;
    println!("Połączono z serwerem");

    let n_shared = Arc::new(Mutex::new(0_u128));
    let e_shared = Arc::new(Mutex::new(0_u128));

    // Odbieranie wiadomości od serwera
    let mut stream_clone = stream.try_clone()?; // Tworzenie klonu strumienia, by odbierać wiadomości od serwera
    thread::spawn(move || {
        let reader = BufReader::new(&mut stream_clone);
        for line in reader.lines() {
            match line {
                Ok(msg) => {
                    if let Some(first_word) = msg.split_whitespace().next() {
                        if let Ok(new_n) = first_word.parse::<u128>() {
                            *n_shared.lock().unwrap() = new_n;
                        }
                    }
                    //let decrypted = unencryption::decrypt(correct_form);
                    println!("Serwer: {}", msg) },
                Err(e) => {
                    println!("Błąd odczytu od serwera: {}", e);
                    break;
                }
            }
        }
    });

    // Wysyłanie wiadomości do serwera
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?; // Operator propagacji błędów
        let encrypted_message = cryptography::encryption::encrypt(&line, e, n);
        writeln!(stream, "{}", n)?;
        writeln!(stream, "{}", e)?;
        for i in encrypted_message.iter() {
            writeln!(stream, "{}", i)?;
        }
    }
    Ok(())
}