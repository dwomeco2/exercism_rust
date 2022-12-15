pub fn square(s: u32) -> u64 {
    match s {
        1 => 1,
        2 => 2,
        n if n < 1 || n > 64 => panic!("Square must be between 1 and 64"),
        n => 2u64.pow(n - 1),
    }
}

pub fn total() -> u64 {
    (1..=64).map(|n| square(n)).sum::<u64>()
}
