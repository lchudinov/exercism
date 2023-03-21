pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for ch in string.chars() {
        let last = stack.last();
        match ch {
            '{' | '[' | '(' => stack.push(ch),
            '}' if last != Some(&'{') => { return false; }
            ')' if last != Some(&'(') => { return false; }
            ']' if last != Some(&'[') => { return false; }
            '}' if last == Some(&'{') => { stack.pop(); }
            ')' if last == Some(&'(') => { stack.pop(); }
            ']' if last == Some(&'[') => { stack.pop(); }
            _ => (),
        }
    }
    stack.is_empty()
}
