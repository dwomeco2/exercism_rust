// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { pos: (x, y), d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot::new(self.pos.0, self.pos.1, new_d)
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Robot::new(self.pos.0, self.pos.1, new_d)
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let new_pos = match self.d {
            Direction::North => (self.pos.0, self.pos.1 + 1),
            Direction::East => (self.pos.0 + 1, self.pos.1),
            Direction::South => (self.pos.0, self.pos.1 - 1),
            Direction::West => (self.pos.0 - 1, self.pos.1),
        };
        Robot::new(new_pos.0, new_pos.1, self.d)
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut s = self;
        for i in instructions.chars() {
            let ns = match i {
                'A' => s.advance(),
                'L' => s.turn_left(),
                'R' => s.turn_right(),
                _ => unreachable!(),
            };
            s = ns
        }
        s
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
