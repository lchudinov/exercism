/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let code = isbn.chars().filter(|&c| c != '-').rev().enumerate().fold(
        0usize,
        |acc, (index, c)| match (index, c) {
            (0, 'X') => 10,
            (n, c) if c.is_ascii_digit() && n < 10 => {
                acc + (n + 1) * (c.to_digit(10).unwrap()) as usize
            }
            _ => 1,
        },
    );
    code > 0 && code % 11 == 0
}
