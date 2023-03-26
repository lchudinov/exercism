pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score: Self::normalize_score(score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = vec![];
        let mut score = self.score;
        while score > 0 {
            if score >= 128 {
                score -= 128;
                allergies.push(Allergen::Cats);
            } else if score >= 64 {
                score -= 64;
                allergies.push(Allergen::Pollen);
            } else if score >= 32 {
                score -= 32;
                allergies.push(Allergen::Chocolate);
            } else if score >= 16 {
                score -= 16;
                allergies.push(Allergen::Tomatoes);
            } else if score >= 8 {
                score -= 8;
                allergies.push(Allergen::Strawberries);
            } else if score >= 4 {
                score -= 4;
                allergies.push(Allergen::Shellfish);
            } else if score >= 2 {
                score -= 2;
                allergies.push(Allergen::Peanuts);
            } else {
                allergies.push(Allergen::Eggs);
                score -= 1;
            }
        }
        allergies
    }

    fn normalize_score(score: u32) -> u32 {
        let mut normalized = score;
        while normalized >= 256 {
            normalized -= 256;
        }
        normalized
    }
}
