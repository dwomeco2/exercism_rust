#[derive(Debug)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: Copy + Clone + PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && self.data.len() == other.data.len()
    }
}

impl<T: Copy + Clone + PartialEq> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        Self {
            data: input.iter().copied().collect(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(&element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|d| other.contains(&d))
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.data.iter().any(|d| other.contains(&d))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .filter(|&d| other.contains(d))
                .copied()
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .filter(|&d| !other.contains(d))
                .copied()
                .collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            data: self
                .difference(&other)
                .data
                .iter()
                .chain(other.data.iter())
                .copied()
                .collect(),
        }
    }
}
