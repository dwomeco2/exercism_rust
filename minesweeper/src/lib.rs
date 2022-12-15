use std::collections::HashMap;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut hm = HashMap::new();
    let (row, col) = get_row_col(minefield);
    let mine_list = get_mine_list(minefield, col);
    let adj_list = mine_list
        .iter()
        .map(|&(x, y)| {
            let x = x as i32;
            let y = y as i32;
            let adj_arr = [
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ];
            return adj_arr;
        })
        .map(|arr| {
            let adj = arr
                .into_iter()
                .filter(|&(x, y)| x >= 0 && y >= 0 && x < row as i32 && y < col as i32)
                .filter(|&(x, y)| !mine_list.contains(&(x as usize, y as usize)))
                .collect::<Vec<_>>();

            return adj;
        })
        .collect::<Vec<_>>();
    adj_list.iter().for_each(|arr| {
        for ele in arr {
            *hm.entry(ele).or_insert(0) += 1
        }
    });
    let mut board = Vec::new();
    for r in 0..row {
        let mut line: String = "".to_owned();
        for c in 0..col {
            if let Some(n) = hm.get(&(r as i32, c as i32)) {
                line.push_str(&n.to_string());
            } else if mine_list.contains(&(r, c)) {
                line.push_str(&"*");
            } else {
                line.push_str(&" ");
            }
        }
        board.push(line);
    }
    board
}

fn get_mine_list(mf: &[&str], col: usize) -> Vec<(usize, usize)> {
    mf.join("")
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == '*' as u8)
        .map(|(i, _)| (i / col, i % col))
        .collect()
}

fn get_row_col(mf: &[&str]) -> (usize, usize) {
    let row = mf.len();
    if row == 0 {
        return (0, 0);
    }
    let col = mf.get(0).unwrap().len();
    (row, col)
}
