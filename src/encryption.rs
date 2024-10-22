use crate::RSA;

pub fn to_ascii(text: &str) -> Vec<u8>
{
    let mut ascii = Vec::new();
    for i in text.chars()
    {
        ascii.push(i as u8);
    }
    return ascii;
}

/*pub fn dividing_into_blocks(mut text: &str) 
{
    let (_, _, n, _) = RSA::generate_rsa_values();
}*/