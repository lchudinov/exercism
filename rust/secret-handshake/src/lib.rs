pub fn actions(n: u8) -> Vec<&'static str> {
    let mut actions : Vec<&'static str> = vec![];
    if n & 0b00001 == 0b00001 {
        actions.push("wink");
    }
    if n & 0b00010 == 0b00010 {
        actions.push("double blink");
    }
    if n & 0b00100 == 0b00100 {
        actions.push("close your eyes");
    }
    if n & 0b01000 == 0b01000 {
        actions.push("jump");
    }
    if n & 0b10000 == 0b10000 {
        actions.reverse();
    }
    actions
}
