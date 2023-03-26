pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let is_yell = trimmed
        .chars()
        .all(|ch| ch.is_uppercase() || !ch.is_alphabetic())
        && trimmed.chars().any(|ch| ch.is_uppercase());
    let is_question = trimmed.ends_with('?');
    let is_silence = trimmed.chars().all(|ch| ch.is_whitespace());

    match (is_yell, is_question, is_silence) {
        (_, _, true) => "Fine. Be that way!",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Sure.",
        (true, _, _) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
