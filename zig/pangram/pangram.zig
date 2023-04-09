const std = @import("std");

pub fn isPangram(str: []const u8) bool {
    var list = std.AutoHashMap(u8, void).init(std.heap.page_allocator);
    defer list.deinit();

    for (str) |byte| {
        if (std.ascii.isAlphabetic(byte)) {
            const letter = std.ascii.toLower(byte);
            list.put(letter, undefined) catch unreachable;
        }
    }
    return list.count() == 26;
}
