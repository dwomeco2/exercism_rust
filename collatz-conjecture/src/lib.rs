pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = 0;
    let mut n = (n != 0).then_some(n);
    while n.is_some() && n? != 1 {
        let nn = n?;
        n = match nn % 2 == 0 {
            true => Some(nn / 2),
            false => nn.checked_mul(3).and_then(|n| n.checked_add(1)),
        };
        steps += 1;
    }
    if n.is_none() {
        return None;
    }
    return Some(steps);
}
