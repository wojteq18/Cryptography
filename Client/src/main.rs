mod db;

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
    let d = unencryption::find_d(e, fi_n);

    // **Arc<Mutex<T>>** dla globalnych wartości kluczy
    let public_n = Arc::new(Mutex::new(n));
    let public_e = Arc::new(Mutex::new(e));

    // 1. Łączenie z serwerem
    let mut stream = TcpStream::connect("127.0.0.1:9999")?;
    writeln!(stream, "{}", n)?;
    writeln!(stream, "{}", e)?;
    println!("Połączono z serwerem");

    // Kopie do wątku odbierającego
    let public_n_clone = Arc::clone(&public_n);
    let public_e_clone = Arc::clone(&public_e);
    let mut stream_clone = stream.try_clone()?;

    // Wątek odbierający dane
    thread::spawn(move || {
        let reader = BufReader::new(&mut stream_clone);
        let mut messages: Vec<u128> = Vec::new();

        for line_result in reader.lines() {
            match line_result {
                Ok(msg) => {
                    // Każda linia interpretowana jako u128
                    if let Ok(value) = msg.trim().parse::<u128>() {
                        if messages.len() < 3 {
                            let correct_form = value as u128;
                            messages.push(correct_form);
                        } else {
                            let correct_form = value as u128;
                            messages.push(correct_form);
                            let length = messages[2];
                            println!("n: {}", messages[0]);
                            println!("e: {}", messages[1]);

                            if messages.len() == (length as usize) + 1 {
                                // **Bezpieczna aktualizacja kluczy**
                                let mut public_n = public_n_clone.lock().unwrap();
                                let mut public_e = public_e_clone.lock().unwrap();
                                *public_n = messages[0];
                                *public_e = messages[1];

                                let decrypted_message = unencryption::decrypt(messages.clone(), n, e, d);
                                println!("Otrzymano wiadomość: {:?}", &decrypted_message);
                                messages.clear();
                            }
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

    // 2. Wysyłanie danych do serwera (szyfrujemy publicznym kluczem (n, e))
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;

        // **Bezpieczny odczyt aktualnych wartości kluczy**
        let public_n = *public_n.lock().unwrap();
        let public_e = *public_e.lock().unwrap();

        // Szyfrujemy wiadomość
        let encrypted_message = encrypt(&line, public_e, public_n);
        let length = encrypted_message.len();
        writeln!(stream, "{}", length)?;
        // Każdy fragment szyfrogramu wysyłamy w osobnej linii
        for part in encrypted_message {
            writeln!(stream, "{}", part)?;
        }
    }
    Ok(())
}
