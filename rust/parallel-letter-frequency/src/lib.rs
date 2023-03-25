use std::{collections::HashMap};
use std::thread;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut m = HashMap::<char, usize>::new();
    let v : Vec<String> = input.iter().map(|&s| s.to_string()).collect();
    let mut pool = vec![];
    for s in v {
        let handle = thread::spawn(move || freq(&s));
        pool.push(handle);
    }
    for handle in pool {
        let mx = handle.join().unwrap();
        for (&key, &value) in mx.iter() {
            m.entry(key)
                .and_modify(|count| *count += value)
                .or_insert(value);
        }
    }
    m
}

fn freq(input: &String) -> HashMap<char, usize> {
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
