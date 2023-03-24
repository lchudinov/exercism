pub fn encode(source: &str) -> String {
    let mut v: Vec<(char, u32)> = vec![];
    for ch in source.chars() {
        if let Some((last_char, count)) = v.last_mut() {
            if *last_char == ch {
                *count += 1;
            } else {
                v.push((ch, 1));
            }
        } else {
            v.push((ch, 1))
        }
    }
    v.iter()
        .map(|(ch, count)| match count {
            1 => format!("{}", ch),
            _ => format!("{}{}", count, ch),
        })
        .collect::<String>()
}

enum Entry {
    Count(u32),
    Char(char),
}
pub fn decode(source: &str) -> String {
    let mut v: Vec<Entry> = vec![];
    for ch in source.chars() {
        if let Some(entry) = v.last_mut() {
            match entry {
                Entry::Count(n) if ch.is_ascii_digit() => *n = *n * 10 + ch.to_digit(10).unwrap(),
                Entry::Char(_) if ch.is_ascii_digit() => {
                    v.push(Entry::Count(ch.to_digit(10).unwrap()))
                }
                _ => v.push(Entry::Char(ch)),
            }
        } else if ch.is_ascii_digit() {
            v.push(Entry::Count(ch.to_digit(10).unwrap()))
        } else {
            v.push(Entry::Char(ch))
        }
    }
    let mut count: u32 = 1;
    let mut result: String = "".to_string();
    for e in v {
        match e {
            Entry::Count(n) => {
                count = n;
            }
            Entry::Char(ch) => {
                result = format!("{}{}", result, format!("{}", ch).repeat(count as usize));
                count = 1;
            }
        }
    }
    result
}
