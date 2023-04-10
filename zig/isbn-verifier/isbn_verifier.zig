const std = @import("std");
const ArrayList = std.ArrayList;

pub fn isValidIsbn10(s: []const u8) bool {
    var sum: usize = 0;
    var pos: usize = 0;
    if (s.len < 10) {
        return false;
    }
    for (s) |c| {
        if (c == '-') {
            continue;
        }
        const mult = 10 - pos;
        if (pos < 9 and !std.ascii.isDigit(c)) {
            return false;
        }
        if (pos == 9 and (c != 'X' and !std.ascii.isDigit(c))) {
            return false;
        }
        if (pos > 9) {
            return false;
        }
        if (pos < 9) {
            sum += (c - '0') * mult;
        } else if (pos == 9 and c == 'X') {
            sum += 10;
        } else if (pos == 9) {
            sum += (c - '0');
        } else {
            return false;
        }
        pos += 1;
    }
    return sum % 11 == 0;
}
