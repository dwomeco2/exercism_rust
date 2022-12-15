pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..=upper_bound).filter(is_prime).collect()
}

fn is_prime(n: &u64) -> bool {
    let mut result = true;

    for i in 1..(n / 2 + 1) {
        if i != 1 && n % i == 0 {
            result = false;

            break;
        }
    }

    result
}
