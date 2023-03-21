pub fn brackets_are_balanced(string: &str) -> bool {
    let mut init = (vec![], true);
    let result = string.chars().fold(&mut init, |acc, ch| {
        match ch {
            '{' | '[' | '(' => acc.0.push(ch),
            '}' => {
                if Some('{') != acc.0.pop() {
                    acc.1 = false
                }
            }
            ']' => {
                if Some('[') != acc.0.pop() {
                    acc.1 = false
                }
            }
            ')' => {
                if Some('(') != acc.0.pop() {
                    acc.1 = false
                }
            }
            _ => {}
        };
        acc
    });
    result.0.len() == 0 && result.1
}
