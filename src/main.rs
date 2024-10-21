mod RSA;
fn main() 
{
    let (p, q, n, fi_n) = RSA::generate_rsa_values();
    println!("p: {}, q: {}, n: {}, fi_n: {}", p, q, n, fi_n);
}
