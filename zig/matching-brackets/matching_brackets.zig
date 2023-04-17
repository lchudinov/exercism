const std = @import("std");
const mem = std.mem;

pub fn isBalanced(allocator: mem.Allocator, s: []const u8) !bool {
    var stack = std.ArrayList(u8).init(allocator);
    defer stack.deinit();
    for (s) |c| {
        switch (c) {
            '[', '{', '(' => try stack.append(c),
            ']', '}', ')' => {
                const last = stack.popOrNull() orelse {
                    return false;
                };
                if (c == ']' and last != '[') {
                    return false;
                }
                if (c == '}' and last != '{') {
                    return false;
                }
                if (c == ')' and last != '(') {
                    return false;
                }
            },
            else => {},
        }
    }
    return stack.items.len == 0;
}
