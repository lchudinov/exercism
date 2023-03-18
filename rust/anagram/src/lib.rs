use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_uppercase = word.to_uppercase();
    let word_uppercase_sorted = sort_string(&word_uppercase);

    possible_anagrams
        .iter()
        .filter(|anagram| {
            let anagram_uppercase = anagram.to_uppercase();
            anagram_uppercase != word_uppercase
                && sort_string(&anagram_uppercase) == word_uppercase_sorted
        })
        .cloned()
        .collect()
}

fn sort_string(s: &String) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v.sort_unstable();
    v.into_iter().collect()
}
