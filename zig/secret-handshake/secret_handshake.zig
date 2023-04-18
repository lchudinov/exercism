const std = @import("std");
const mem = std.mem;

pub const Signal = enum {
    wink,
    double_blink,
    close_your_eyes,
    jump,
};

pub fn calculateHandshake(allocator: mem.Allocator, number: isize) mem.Allocator.Error![]const Signal {
    var actions = std.ArrayList(Signal).init(allocator);
    defer actions.deinit();
    if (number & 0b0001 == 0b0001) {
        try actions.append(Signal.wink);
    }
    if (number & 0b0010 == 0b0010) {
        try actions.append(Signal.double_blink);
    }
    if (number & 0b0100 == 0b0100) {
        try actions.append(Signal.close_your_eyes);
    }
    if (number & 0b1000 == 0b1000) {
        try actions.append(Signal.jump);
    }
    if (number & 0b10000 == 0b10000 and actions.items.len > 0) {
        mem.reverse(Signal, actions.items);
    }
    return actions.toOwnedSlice();
}
