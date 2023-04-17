const std = @import("std");
const mem = std.mem;

pub fn abbreviate(allocator: mem.Allocator, words: []const u8) mem.Allocator.Error![]u8 {
    var list = std.ArrayList(u8).init(allocator);
    defer list.deinit();
    var iter = mem.tokenize(u8, words, " \n\r\t_-");
    while (iter.next()) |word| {
        try list.append(std.ascii.toUpper(word[0]));
    }
    return list.toOwnedSlice();
}
