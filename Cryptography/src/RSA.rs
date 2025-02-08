use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
pub fn is_prime(n: u128) -> bool
{
    if n <= 1
    {
        return false
    }

    for i in 2..=((n as f64).sqrt() as u128)
    {
        if n % i == 0
        {
            return false;
        }
    }
    return true;
}

pub fn nwd(mut a: u128, mut b: u128) -> u128
{
    while b != 0
    {
        let c = a % b;
        a = b;
        b = c;
    }
    return a;
}

pub fn generate_p() -> u128
{
    let mut rng = ChaCha8Rng::from_entropy();
    loop
    {
        let p: u128 = rng.gen_range(100..10000);
        if is_prime(p)
        {
            return p;
        }
    }
}

pub fn generate_rsa_values() -> (u128, u128, u128, u128) //funkcja zwraca wartości w kolejności: (p, q, n, fi_n)
{
    let p = generate_p();
    let q = generate_p();
    let n = p * q;
    let fi_n = (p - 1) * (q - 1);
    return (p, q, n, fi_n);
}

pub fn generate_e(fi_n: u128) -> u128
{
    let mut rng = ChaCha8Rng::from_entropy();

    loop
    {
        let e: u128 = rng.gen_range(2..fi_n);
        if nwd(fi_n, e) == 1
        {
            return e;
        }
    }
}
