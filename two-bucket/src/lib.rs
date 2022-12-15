#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut cycle: Vec<u8> = vec![];

    let (mut b1, mut b2, mut moves) = (0u8, 0u8, 0u8);
    let (cap1, cap2, bucket1, bucket2) = match &start_bucket {
        &Bucket::One => (capacity_1, capacity_2, Bucket::One, Bucket::Two),
        &Bucket::Two => (capacity_2, capacity_1, Bucket::Two, Bucket::One),
    };

    while b1 != goal && b2 != goal {
        if b1 == 0 {
            b1 = cap1;
        } else if cap2 == goal {
            b2 = cap2;
        } else if b2 < cap2 {
            move2(&mut b1, &mut b2, cap2);
            if cycle.contains(&b1) {
                return None;
            } else if b1 != 0 {
                cycle.push(b1);
            }
        } else if b2 == cap2 {
            b2 = 0;
        } else {
            unreachable!()
        }
        moves += 1;
    }

    return Some(BucketStats {
        moves,
        goal_bucket: if b1 == goal { bucket1 } else { bucket2 },
        other_bucket: if b1 == goal { b2 } else { b1 },
    });
}

fn move2(b1: &mut u8, b2: &mut u8, cap: u8) {
    if *b1 + *b2 <= cap {
        *b2 += *b1;
        *b1 = 0;
    } else {
        *b1 = *b1 - (cap - *b2);
        *b2 = cap;
    }
}
