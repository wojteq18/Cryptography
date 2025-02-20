use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{str, thread};
use std::process::{Command, Stdio};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

//funkcja obsługująca klienta
fn handle_client(
    stream: TcpStream, 
    clients: Arc<Mutex<Vec<Arc<TcpStream>>>>, 
) {
    //peer_addr - adres klienta
    let peer_addr = stream.peer_addr().unwrap();

    //kopiujemy stream dla różnych części kodu
    let stream_reader = stream.try_clone().unwrap();
    let stream_writer = Arc::new(stream.try_clone().unwrap());

    //reader - bufor do odczytu danych od klienta
    let mut reader = BufReader::new(stream_reader);

    //zmienna do przechowywanie pojedyńczej wiadomości od klienta
    let mut buffer = String::new();

    //dodanie klienta do listy
    {
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.push(stream_writer);
    }

    println!("Nowy klient połączony!");

    loop {
        buffer.clear();

        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }

            Ok(_) => {
                let message = buffer.trim();
                println!("Otrzymano wiadomość od {}: {}", peer_addr, message);

                //Rozsyłaanie wiadomości do wszystkich klientów
                let clients_guard = clients.lock().unwrap();
                for client in clients_guard.iter() {
                    let mut client_stream = client.try_clone().unwrap();
                    if client.peer_addr().unwrap() != peer_addr {
                        writeln!(client_stream, "{}", message).unwrap();
                    }
                }
            }

            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
    // po zakończeniu połączenia usuwamy klienta z listy
    {
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.retain(|client| client.peer_addr().unwrap() != peer_addr);
    }
    println!("Klient usunięty z listy!");
}

//funckja inicjalizująca serwer
fn initialize_server(adress: &str) -> std::io::Result<TcpListener> {
    let listener = TcpListener::bind(adress)?;
    println!("Serwer uruchomiony na {}", adress);
    Ok(listener)
}

fn main() {
    println!("zaczynamy...");
    let lst = initialize_server("127.0.0.1:9999").expect("Nie udało się zainicjować serwera");
    
    //wspódzielona lista klientów
    let clients: Arc<Mutex<Vec<Arc<TcpStream>>>> = Arc::new(Mutex::new(Vec::new()));


    for stream in lst.incoming() {
        match stream {
            Ok(stream) => {
                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || {
                    handle_client(stream, clients_clone);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
