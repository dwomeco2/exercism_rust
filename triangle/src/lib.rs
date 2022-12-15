use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Clone + Copy + PartialOrd<T> + PartialEq<T> + Add<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let mut sides = sides.clone();
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        (sides[0] + sides[1] > sides[2]).then_some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2]
    }
}
