pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // 3 6 9 5
    // 4 8 12 6 12
    let mut v = factors
        .iter()
        .filter(|&&n| n != 0)
        .map(|&n| (n..).step_by(n as usize))
        .map(|it| it.take_while(|&x| x < limit).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    println!("{:?}", v);
    v.sort();
    v.dedup();
    v.iter().sum::<u32>()
}
