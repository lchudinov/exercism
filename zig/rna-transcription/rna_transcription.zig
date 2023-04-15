// Import the appropriate standard library and modules
const std = @import("std");
const mem = std.mem;

const RnaError = error{InvalidNucleotide};

pub fn toRna(allocator: mem.Allocator, dna: []const u8) (mem.Allocator.Error || RnaError)![]const u8 {
    var list = std.ArrayList(u8).init(allocator);
    defer list.deinit();
    for (dna) |n| {
        const c = try translate(n);
        try list.append(c);
    }
    return list.toOwnedSlice();
}

fn translate(c: u8) RnaError!u8 {
    return switch (c) {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        else => RnaError.InvalidNucleotide,
    };
}
