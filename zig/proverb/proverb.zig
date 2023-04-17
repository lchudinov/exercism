const std = @import("std");
const mem = std.mem;
const fmt = std.fmt;

pub fn recite(allocator: mem.Allocator, words: []const []const u8) (fmt.AllocPrintError || mem.Allocator.Error)![][]u8 {
    var sentences = std.ArrayList([]u8).init(allocator);
    defer sentences.deinit();
    var i: usize = 0;
    if (words.len > 0) {
        while (i < words.len - 1) {
            const sentence = try std.fmt.allocPrint(allocator, "For want of a {s} the {s} was lost.\n", .{ words[i], words[i + 1] });
            try sentences.append(sentence);
            i += 1;
        }
        const lastSentence = try std.fmt.allocPrint(allocator, "And all for the want of a {s}.\n", .{words[0]});
        try sentences.append(lastSentence);
    }
    return sentences.toOwnedSlice();
}
