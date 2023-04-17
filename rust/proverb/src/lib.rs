
pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::from("");
    }
    let words = list.to_vec();
    let mut sentences = words
    .windows(2)
        .map(|w| {format!("For want of a {} the {} was lost.\n", w[0], w[1])})
        .collect::<Vec<String>>();
    sentences.push(format!("And all for the want of a {}.", words[0]));
    sentences.join("")
}
