pub fn private_key(p: u64) -> u64 {
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow_mod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow_mod(b_pub, a, p)
}

fn pow_mod(base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let (mut base, modulus, mut result) = (base as u128, modulus as u128, 1u128);
    base = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus;
    }

    return result as u64;
}
