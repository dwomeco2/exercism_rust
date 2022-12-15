pub fn factors(mut n: u64) -> Vec<u64> {
    let mut out = vec![];

    for i in 2..=((n / 2) + 1) {
        while n % i == 0 {
            out.push(i);
            n /= i;
        }

        if n == 1 {
            break;
        }
    }
    out
}
