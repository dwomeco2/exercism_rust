pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score: (score as u8) as u32,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;
        [
            Allergen::Cats,
            Allergen::Pollen,
            Allergen::Chocolate,
            Allergen::Tomatoes,
            Allergen::Strawberries,
            Allergen::Shellfish,
            Allergen::Peanuts,
            Allergen::Eggs,
        ]
        .into_iter()
        .filter(|x| {
            let f = score >= *x as u32;
            if f {
                score -= *x as u32;
            }
            f
        })
        .collect::<Vec<Allergen>>()
    }
}
