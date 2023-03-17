use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut word_vec_uppercase = to_vec_uppercase(word);
    let word_uppercase_string = vec_uppercase_to_string(&word_vec_uppercase);
    word_vec_uppercase.sort_unstable();
    let word_uppercase_sorted = vec_uppercase_to_string(&word_vec_uppercase);

    possible_anagrams
        .iter()
        .filter(|anagram| check_if_anagram(anagram, &word_uppercase_string, &word_uppercase_sorted))
        .map(|&s| s)
        .collect()
}

fn check_if_anagram(
    anagram: &str,
    word_uppercase_string: &String,
    word_uppercase_sorted: &String,
) -> bool {
    let mut anagram_vec_uppercase = to_vec_uppercase(anagram);
    let anagram_uppercase_string = vec_uppercase_to_string(&anagram_vec_uppercase);
    if anagram_uppercase_string == *word_uppercase_string {
        false
    } else {
        anagram_vec_uppercase.sort_unstable();
        let anagram_uppercase_sorted = vec_uppercase_to_string(&anagram_vec_uppercase);
        anagram_uppercase_sorted == *word_uppercase_sorted
    }
}

fn to_vec_uppercase(s: &str) -> Vec<char> {
    s.chars().map(|c| c.to_uppercase()).flatten().collect()
}

fn vec_uppercase_to_string(v: &Vec<char>) -> String {
    v.into_iter().collect()
}
