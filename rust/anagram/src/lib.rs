use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_uppercase_sorted = to_sorted_uppercase(word);
    let word_uppercase = word.to_uppercase();

    possible_anagrams
        .into_iter()
        .filter(|anagram| check_if_anagram(anagram, &word_uppercase, &word_uppercase_sorted))
        .map(|&s| s)
        .collect()
}

fn check_if_anagram(
    anagram: &str,
    word_uppercase: &String,
    word_uppercase_sorted: &String,
) -> bool {
    anagram.to_uppercase() != *word_uppercase
        && to_sorted_uppercase(anagram) == *word_uppercase_sorted
}

fn to_sorted_uppercase(s: &str) -> String {
    let mut v: Vec<char> = s.chars().map(|c| c.to_uppercase()).flatten().collect();
    v.sort_unstable();
    v.into_iter().collect()
}
