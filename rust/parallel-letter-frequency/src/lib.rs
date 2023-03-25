use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut m = HashMap::<char, usize>::new();
    for &s in input {
        for ch in s
            .chars()
            .filter(|ch| ch.is_alphabetic())
            .flat_map(|ch| ch.to_lowercase())
        {
            m.entry(ch).and_modify(|count| *count += 1).or_insert(1);
        }
    }
    m
}
