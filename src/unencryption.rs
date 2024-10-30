use crate::encryption;

pub fn find_d(e: u128, fi_n: u128) -> u128
{
    let mut d = 1;
    while (d * e) % fi_n != 1
    {
        d = d + 1;
    }
    return d;
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

