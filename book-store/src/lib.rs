const SET_PRICES: [u32; 6] = [
    0,    //
    800,  // no discount
    1520, // 800 * 2 * .95
    2160, // 800 * 3 * .90
    2560, // 800 * 4 * .80
    3000, // 800 * 5 * .75
];

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut sets = Vec::<Vec<u32>>::new();

    for &book in books {
        if let Some(set) = sets
            .iter_mut()
            .filter(|set| !set.contains(&book))
            .min_by_key(|set| SET_PRICES[set.len() + 1] - SET_PRICES[set.len()])
        {
            set.push(book);
        } else {
            sets.push(vec![book]);
        }
    }

    sets.into_iter().map(|set| SET_PRICES[set.len()]).sum()
}
