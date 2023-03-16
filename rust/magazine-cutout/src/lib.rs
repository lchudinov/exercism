// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::<&str, u32>::new();
    for word in magazine.iter() {
        *words.entry(word).or_insert(0) += 1;
    }
    note.iter().all(|&n| {
        let count = words.entry(n).or_default();
        match *count {
            0 => false,
            _ => {
                *count -= 1;
                true
            }
        }
    })
}
