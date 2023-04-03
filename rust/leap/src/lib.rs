pub fn is_leap_year(year: u64) -> bool {
    let by_4 = year % 4 == 0;
    let by_100 = year % 100 == 0;
    let by_400 = year % 400 == 0;

    match (by_4, by_100, by_400) {
        (false, _, _) => false,
        (true, false, true) => true,
        (true, false, false) => true,
        (true, true, true) => true,
        _ => false,
    }
}
