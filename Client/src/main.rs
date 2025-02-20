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

    // 1. Łączenie z serwerem
    let mut stream = TcpStream::connect("127.0.0.1:9999")?;
    println!("Połączono z serwerem");

    // Wątek odbierający dane
    let mut stream_clone = stream.try_clone()?;
    thread::spawn(move || {
        let reader = BufReader::new(&mut stream_clone);
        // Wszystkie odebrane wartości (jako u128) wrzucamy do wektora
        let mut messages: Vec<u128> = Vec::new();

        for line_result in reader.lines() {
            match line_result {
                Ok(msg) => {
                    // Każda linia interpretowana jako u128
                    if let Ok(value) = msg.trim().parse::<u128>() {
                        messages.push(value);
                        //println!("Odebrano: {:?}", messages);
                        let length = messages[0];
                        if length == (messages.len() + 3) as u128 {
                            println!("Otrzymano wiadomość: {:?}", messages);
                            messages.clear();

                        }
                    } else {
                        println!("Błąd parsowania w linii: {}", msg);
                    }
                },
                Err(e) => {
                    eprintln!("Błąd odczytu od serwera: {}", e);
                    break;
                }
            }
        }
    });

    // 2. Wysyłanie danych do serwera (szyfrujemy lokalnym kluczem (n, e))
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        // Szyfrujemy wiadomość
        let encrypted_message = encrypt(&line, e, n);

        // Każdy fragment szyfrogramu wysyłamy w osobnej linii
        for part in encrypted_message {
            writeln!(stream, "{}", part)?;
        }
    }
    Ok(())
}
