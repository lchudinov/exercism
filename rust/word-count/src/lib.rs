use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    words
        .replace(
            ['\"', ',', '&', '.', '!', '?', '@', '$', '%', ':', '^'],
            " ",
        )
        .split_ascii_whitespace()
        .map(|word| word.trim_matches('\''))
        .filter(|word| !word.is_empty())
        .map(|word| word.to_ascii_lowercase())
        .for_each(|word| {
            map.entry(word).and_modify(|count| *count += 1).or_insert(1);
        });
    map
}
