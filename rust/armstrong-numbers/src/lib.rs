pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let power = str.len() as u32;
    let result = str
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .fold(0u64, |acc, x| acc + (x as u64).pow(power));
    result == num as u64
}
