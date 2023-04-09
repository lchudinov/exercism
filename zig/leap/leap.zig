pub fn isLeapYear(year: u32) bool {
    const div_4 = year % 4 == 0;
    const div_400 = year % 400 == 0;
    const div_100 = year % 100 == 0;
    if (div_400) {
        return true;
    }
    if (div_4 and !div_100) {
        return true;
    }
    return false;
}
