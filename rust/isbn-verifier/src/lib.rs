/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let code = isbn.chars().filter(|&c| c != '-').rev().enumerate().fold(
        Some(0usize),
        |acc, (index, c)| match (index, c, acc) {
            (0, 'X', _) => Some(10),
            (n, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9', Some(a)) if n < 10 => {
                Some(a + (n + 1) * (c.to_digit(10).unwrap()) as usize)
            }
            _ => None,
        },
    );
    matches!(code, Some(n) if n > 0 && n % 11 == 0)
}
