pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let rows_len = input.len();
    let cols_len = input.first().unwrap_or(&vec![]).len();
    input
        .iter()
        .enumerate()
        .map(|(row, inner)| {
            inner
                .iter()
                .enumerate()
                .filter(|(col, num)| {
                    (0..cols_len).all(|c| input[row][c] <= **num)
                        && (0..rows_len).all(|r| input[r][*col] >= **num)
                })
                .map(|(col, _num)| (row, col))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}
