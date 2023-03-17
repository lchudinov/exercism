#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Value(value) => stack.push(value.clone()),
            _ => {
                let first = stack.pop();
                let second = stack.pop();
                match (first, second) {
                    (Some(a), Some(b)) => evaluate_op(&mut stack, input, a, b),
                    _ => return None,
                }
            }
        }
    }
    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}

fn evaluate_op(
    stack: &mut Vec<i32>,
    op: &CalculatorInput,
    a: i32,
    b: i32,
) {
    match op {
        CalculatorInput::Add => stack.push(b + a),
        CalculatorInput::Subtract => stack.push(b - a),
        CalculatorInput::Multiply => stack.push(b * a),
        CalculatorInput::Divide => stack.push(b / a),
        _ => return
    }
}
