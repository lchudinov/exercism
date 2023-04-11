const std = @import("std");

pub fn isValid(s: []const u8) bool {
    var i: usize = 0;
    for (s) |c| {
        if (c != ' ') {
            i += 1;
        }
    }
    const lenWithoutSpaces = i;
    if (lenWithoutSpaces < 2) {
        return false;
    }
    const lastIndex: i32 = @intCast(i32, s.len) - 1;
    var index: i32 = @intCast(i32, lastIndex);
    var counter: usize = 0;
    var sum: usize = 0;
    while (index >= 0) {
        const c = s[@intCast(usize, index)];
        if (c == ' ') {
            index -= 1;
            continue;
        }
        if (!std.ascii.isDigit(c)) {
            return false;
        }
        var digit: u8 = c - '0';
        if (counter % 2 == 1) {
            digit *= 2;
            if (digit > 9) {
                digit -= 9;
            }
        }
        sum += digit;
        index -= 1;
        counter += 1;
    }
    return sum % 10 == 0;
}
