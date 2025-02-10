use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use std::thread;
use cryptography::unencryption;
use cryptography::encryption::{self, encrypt};
use cryptography::RSA;

fn string_to_vec_u128(s: &str) -> Vec<u128> {
    s.chars().map(|c| c as u128).collect()
}
fn main() -> std::io::Result<()> {
    // Łączenie z serwerem
    let mut stream = TcpStream::connect("127.0.0.1:9999")?;
    println!("Połączono z serwerem");

    // Odbieranie wiadomości od serwera
    let mut stream_clone = stream.try_clone()?; // Tworzenie klonu strumienia, by odbierać wiadomości od serwera
    thread::spawn(move || {
        let reader = BufReader::new(&mut stream_clone);
        for line in reader.lines() {
            match line {
                Ok(msg) => {
                    let correct_form = string_to_vec_u128(&msg);
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
        //generujemy odpowiednie wartości zgodnie z algorytmem RSA
        let (p, q, n, fi_n) = RSA::generate_rsa_values();
        let e = RSA::generate_e(fi_n);
        let encrypted_message = cryptography::encryption::encrypt(&line, e, n);
        stream.write_all(&n.to_le_bytes())?;
        //stream.write_all(&encrypted_message)?; TODO
        stream.write_all(b"\n")?; // Znak nowej linii wymagany przez serwer
    }
    Ok(())
}