#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (a, b) if a > b => if first_list.windows(b).any(|w| w == second_list) {Comparison::Superlist} else {Comparison::Unequal},
        (a, b) if a < b => if second_list.windows(a).any(|w| w == first_list) {Comparison::Sublist} else {Comparison::Unequal},
        (_, _) => if first_list == second_list {Comparison::Equal} else {Comparison::Unequal},
    }
}
