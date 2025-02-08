pub fn to_ascii(text: &str) -> Vec<u8>
{
    let mut ascii = Vec::new();
    for i in text.chars()
    {
        ascii.push(i as u8);
    }
    return ascii;
}

/*pub fn dividing_into_blocks(mut text: &str) -> Vec<String>
{
    let mut blocks: Vec<String> = Vec::new();
    let (_, _, n, _) = RSA::generate_rsa_values();
    for i in text.chars()
    {
        blocks.push(i.to_string());
    }      
    return blocks;
}*/

pub fn encrypt(text: &str, e: u128, n: u128) -> Vec<u128>
{
    let mut encrypted: Vec<u128> = Vec::new();
    let ascii = to_ascii(text);
    for &ascii in ascii.iter()
    {
        let encrypted_block = mod_exp(ascii as u128, e, n);
        encrypted.push(encrypted_block);

    }
    return encrypted;
}

pub fn mod_exp(mut base: u128, mut exp: u128, modulus: u128) -> u128 //funkcja szybkiego potęgowania modularnego: (m^e % n)
{
    let mut result = 1;
    base = base % modulus;
    while exp > 0 
    {
        if exp % 2 == 1 
        {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    return result;
}

