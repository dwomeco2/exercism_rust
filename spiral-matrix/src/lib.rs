enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut current = (0usize, 0usize);
    let mut direction = Direction::Right;

    let mut vec = vec![vec![0; size as usize]; size as usize];
    if size > 0 {
        vec[0][0] = 1;
    }
    for num in 2..=size.pow(2) {
        // next position not exceeding boundary
        // next position not in hashmap
        let mut x = next_position(&current, &direction, size as usize);
        if x.is_none() || !is_empty(&x, &vec) {
            direction = next_direction(&direction);
            x = next_position(&current, &direction, size as usize);
        }

        if let Some(x) = x {
            current = x;
            vec[x.0][x.1] = num;
        }
    }
    vec
}

fn is_empty(x: &Option<(usize, usize)>, vec: &Vec<Vec<u32>>) -> bool {
    if let Some(g) = x {
        return vec[g.0][g.1] == 0;
    }

    return true;
}

fn next_position(current: &(usize, usize), d: &Direction, b: usize) -> Option<(usize, usize)> {
    match d {
        Direction::Right => (current.1 + 1 < b).then_some((current.0, current.1 + 1)),
        Direction::Down => (current.0 + 1 < b).then_some((current.0 + 1, current.1)),
        Direction::Left if (current.1 as i32) - 1 < 0 => None,
        Direction::Left => Some((current.0, current.1 - 1)),
        Direction::Up if (current.0 as i32) - 1 < 0 => None,
        Direction::Up => Some((current.0 - 1, current.1)),
    }
}

fn next_direction(d: &Direction) -> Direction {
    match d {
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
    }
}
