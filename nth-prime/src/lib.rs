fn is_prime(n: &u32) -> bool {
    let mut result = true;

    for i in 1..(n / 2 + 1) {
        if i != 1 && n % i == 0 {
            result = false;

            break;
        }
    }

    result
}

pub fn nth(n: u32) -> u32 {
    let n = n + 1;

    let num = 2..;

    num.filter(is_prime)
        .skip((n - 1) as usize)
        .take(1)
        .next()
        .unwrap()
}
