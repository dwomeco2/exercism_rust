pub fn count(lines: &[&str]) -> u32 {
    let all_corner = find_all_corner(lines);
    all_corner
        .iter()
        .map(|origin| {
            all_corner
                .iter()
                .filter(|corner| {
                    return corner.0 > origin.0 && corner.1 > origin.1;
                })
                .map(|corner| [origin, corner])
                .filter(|pair| is_valid_rectangle(pair, &all_corner, lines))
                .count()
        })
        .sum::<usize>() as u32
}

fn find_all_corner(lines: &[&str]) -> Vec<(usize, usize)> {
    lines
        .iter()
        .enumerate()
        .map(|(y, &ys)| {
            ys.char_indices()
                .filter(|(_x, xs)| xs == &'+')
                .map(|(x, _xs)| (x, y))
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty())
        .flatten()
        .collect::<Vec<_>>()
}

fn is_valid_rectangle(
    corner: &[&(usize, usize); 2],
    all: &Vec<(usize, usize)>,
    lines: &[&str],
) -> bool {
    all_corner_exists(&corner, all) && all_corner_connected(&corner, lines)
}

fn all_corner_exists(corner: &[&(usize, usize); 2], all: &Vec<(usize, usize)>) -> bool {
    let [&(x1, y1), &(x2, y2)] = corner;
    let rec_corners = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];
    return rec_corners.iter().all(|t| all.contains(t));
}

fn all_corner_connected(corner: &[&(usize, usize); 2], lines: &[&str]) -> bool {
    let [&(x1, y1), &(x2, y2)] = corner;
    let it1 = (y1 + 1..y2).map(|y| (x1, y));
    let it2 = (y1 + 1..y2).map(|y| (x2, y));
    let it3 = (x1 + 1..x2).map(|x| (x, y1));
    let it4 = (x1 + 1..x2).map(|x| (x, y2));
    it1.chain(it2).all(|(x, y)| {
        let c = lines.get(y).unwrap().as_bytes()[x];
        c != b' ' && c != b'-'
    }) && it3.chain(it4).all(|(x, y)| {
        let c = lines.get(y).unwrap().as_bytes()[x];
        c != b' ' && c != b'|'
    })
}
