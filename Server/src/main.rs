use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{str, thread};
use std::process::{Command, Stdio};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn handle_client(
    stream: TcpStream, 
    clients: Arc<Mutex<Vec<TcpStream>>>, 
    current_client: Arc<Mutex<usize>>
) {
    
}

fn main() {
    println!("Hello, world!");
}
