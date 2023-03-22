#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    vec: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: std::cmp::PartialEq + std::clone::Clone + std::cmp::Ord,
{
    pub fn new(input: &[T]) -> Self {
        let mut v: Vec<_> = input.to_vec();
        v.sort_unstable();
        v.dedup();
        CustomSet { vec: v }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.vec.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.vec.push(element);
            self.vec.sort_unstable();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.vec.iter().all(|x| other.contains(x))
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.vec.iter().all(|x| !other.contains(x)) && other.vec.iter().all(|x| !self.contains(x))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet {
            vec: self
                .vec
                .iter()
                .cloned()
                .filter(|x| other.contains(x))
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        CustomSet {
            vec: self
                .vec
                .iter()
                .cloned()
                .filter(|x| !other.contains(x))
                .collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut v: Vec<_> = Vec::with_capacity(self.vec.len() + other.vec.len());
        v.extend_from_slice(&self.vec);
        v.extend_from_slice(&other.vec);
        v.sort_unstable();
        v.dedup();
        CustomSet { vec: v }
    }
}
