use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut m = HashMap::<char, usize>::new();

    let v: Vec<_> = input.par_iter().map(|&s| freq(s)).collect();
    for hm in v {
        for (key, value) in hm {
            m.entry(key)
                .and_modify(|count| *count += value)
                .or_insert(value);
        }
    }
    m
}

fn freq(input: &str) -> HashMap<char, usize> {
    let mut m = HashMap::<char, usize>::new();
    for ch in input
        .chars()
        .filter(|ch| ch.is_alphabetic())
        .flat_map(|ch| ch.to_lowercase())
    {
        m.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }
    m
}
