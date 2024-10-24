mod RSA;
mod encryption;
fn main() 
{
    let (p, q, n, fi_n) = RSA::generate_rsa_values();
    println!("p: {}, q: {}, n: {}, fi_n: {}", p, q, n, fi_n);

    println!("Wprowadź wiadomość, którą chcesz zaszyfrować: ");
    let mut massage = String::new();
    std::io::stdin().read_line(&mut massage).expect("Błąd wczytyania danych");
    let massage = massage.trim();
    let ascii = encryption::to_ascii(massage);
    let encryption = encryption::encrypt(massage);
    println!("Wiadomość w postaci ASCII: {:?}", ascii);
    println!("Zaszyfrowana wiadomość: {:?}", encryption);
}
