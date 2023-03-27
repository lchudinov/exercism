enum State {
    Lowercase,
    Uppercase,
    Whitespace,
}
pub fn abbreviate(phrase: &str) -> String {
    let mut result = "".to_string();
    let mut state = State::Whitespace;
    for c in phrase.chars() {
        let is_whitespace = c.is_whitespace() || c == '-' || c == '_';
        let is_uppercase = c.is_ascii_uppercase();
        let is_lowercase = c.is_ascii_lowercase();
        match state {
            State::Uppercase if is_whitespace => {
                state = State::Whitespace;
            }
            State::Uppercase if is_lowercase => {
                state = State::Lowercase;
            }
            State::Uppercase => {
                state = State::Uppercase;
            }
            State::Lowercase if is_whitespace => {
                state = State::Whitespace;
            }
            State::Lowercase if is_uppercase => {
                state = State::Uppercase;
                result.push(c);
            }
            State::Whitespace if is_whitespace => {}
            State::Whitespace if is_uppercase => {
                result.push(c);
                state = State::Uppercase;
            }
            State::Whitespace if is_lowercase => {
                result.push(c.to_ascii_uppercase());
                state = State::Uppercase;
            }
            _ => {}
        }
    }
    result
}
