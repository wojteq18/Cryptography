mod RSA;
mod encryption;
mod unencryption;
fn main() 
{
    let (p, q, n, fi_n) = RSA::generate_rsa_values();
    println!("p: {}, q: {}, n: {}, fi_n: {}", p, q, n, fi_n);
    let (e) = RSA::generate_e(fi_n);
    println!("Wprowadź wiadomość, którą chcesz zaszyfrować: ");
    let mut massage = String::new();
    std::io::stdin().read_line(&mut massage).expect("Błąd wczytyania danych");
    let massage = massage.trim();
    let ascii = encryption::to_ascii(massage);
    let encryption = encryption::encrypt(massage, e, n);
    println!("Wiadomość w postaci ASCII: {:?}", ascii);
    println!("Zaszyfrowana wiadomość: {:?}", encryption);
    let mut d: u128 = unencryption::find_d(e, fi_n);
    let decryption = unencryption::decrypt(encryption, n, e, d);
    println!("Odszyfrowana wiadomość: {:?}", decryption);
}
