/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let stripped = code.replace(" ", "");
    if stripped.len() < 2 {
        return false;
    }
    if stripped.chars().any(|ch| !ch.is_digit(10)) {
        return false;
    }
    let sum = stripped
        .chars()
        .rev()
        .enumerate()
        .map(|(index, ch)| (index, ch.to_digit(10).unwrap()))
        .map(|(index, digit)| if index % 2 == 0 { digit } else { digit * 2 })
        .map(|num| if num > 9 { num - 9 } else { num })
        .fold(0, |sum, num| sum + num);
    let valid = sum % 10 == 0;
    valid
}
