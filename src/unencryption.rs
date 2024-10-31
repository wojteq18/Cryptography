use crate::encryption;

fn extended_gcd(a: u128, b: u128) -> (u128, i128, i128) 
{
    if a == 0 
    {
        return (b, 0, 1);
    } 
    else 
    {
        let (gcd, x1, y1) = extended_gcd(b % a, a);
        let x = y1 - (b / a) as i128 * x1;
        let y = x1;
        return (gcd, x, y);
    }
}
pub fn find_d(e: u128, fi_n: u128) -> u128
{
    let (gcd, x, _) = extended_gcd(e, fi_n);
    if gcd != 1
    {
        panic! ("e i fi_n nie są względnie pierwsze!");
    }
    let d = (x % fi_n as i128 + fi_n as i128) % fi_n as i128;
    return d as u128;
}

pub fn decrypt(encrypted: Vec<u128>, n: u128, e: u128, d: u128) -> String
{
    let mut decrypted = String::new();
    for &encrypted_block in encrypted.iter()
    {
        let decrypted_block = encryption::mod_exp(encrypted_block, d, n);
        decrypted.push(decrypted_block as u8 as char); // Convert decrypted block to character
    }
    return decrypted;
}

