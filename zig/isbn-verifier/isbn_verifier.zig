const std = @import("std");
const ArrayList = std.ArrayList;

pub fn isValidIsbn10(s: []const u8) bool {
    var iter = std.mem.split(u8, s, "-");
    var list = ArrayList(u8).init(std.heap.page_allocator);
    defer list.deinit();
    while (iter.next()) |chunk| {
        _ = list.writer().write(chunk) catch unreachable;
    }
    const s2 = list.items;
    if (s2.len != 10) {
        return false;
    }
    var sum: usize = 0;
    for (s2, 0..) |c, i| {
        const mult = 10 - i;
        if (i < 9 and !std.ascii.isDigit(c)) {
            return false;
        }
        if (i == 9 and (c != 'X' and !std.ascii.isDigit(c))) {
            return false;
        }
        if (i < 9) {
            sum += (c - '0') * mult;
        } else if (c == 'X') {
            sum += 10;
        } else {
            sum += (c - '0');
        }
    }
    return sum % 11 == 0;
}
