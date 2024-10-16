fn cesar_cipher(element: &char) -> char
{
    let alfabhet_upper = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
                    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
                    'U', 'V', 'W', 'X', 'Y', 'Z'];

    let alfabhet_lower = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
                    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
                    'u', 'v', 'w', 'x', 'y', 'z'];

    for i in 0..alfabhet_lower.len()
    {
        if element == &alfabhet_lower[i]
        {
            return alfabhet_lower[(i+3)%26];
        }
        else if element == &alfabhet_upper[i]
        {
            return alfabhet_upper[(i+3)%26];
        }        
    }
    return *element
}

fn main()
{
    let c = 'a';
    println!("{}", cesar_cipher(&c));
}