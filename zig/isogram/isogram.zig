const std = @import("std");

pub fn isIsogram(str: []const u8) bool {
    var map = std.AutoArrayHashMap(u8, void).init(std.heap.page_allocator);
    defer map.deinit();
    for (str) |byte| {
        if (std.ascii.isAlphabetic(byte)) {
            const lower = std.ascii.toLower(byte);
            if (map.contains(lower)) {
                return false;
            }
            map.put(lower, undefined) catch unreachable;
        }
    }
    return true;
}
