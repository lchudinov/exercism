use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_uppercase_sorted = to_uppercase_sorted(word);
    let word_uppercase = word.to_uppercase();

    possible_anagrams
        .iter()
        .filter(|anagram| {
            anagram.to_uppercase() != word_uppercase
                && to_uppercase_sorted(anagram) == word_uppercase_sorted
        })
        .cloned()
        .collect()
}

fn to_uppercase_sorted(s: &str) -> String {
    let mut v: Vec<char> = s.chars().map(|c| c.to_uppercase()).flatten().collect();
    v.sort_unstable();
    v.into_iter().collect()
}
