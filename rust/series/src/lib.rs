pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![String::from(""); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|c| String::from_iter(c.iter()))
        .collect()
}
