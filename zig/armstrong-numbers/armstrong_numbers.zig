const std = @import("std");

pub fn isArmstrongNumber(num: u128) bool {
    var n = num;
    var count: u128 = 1;
    while (n > 9) {
        n /= 10;
        count += 1;
    }
    std.debug.print("count is {}\n", .{count});
    n = num;
    var sum: u128 = 0;
    while (n > 9) {
        const digit = n % 10;
        n /= 10;
        sum += std.math.powi(u128, digit, count) catch unreachable;
    }
    sum += std.math.powi(u128, n, count) catch unreachable;
    std.debug.print("sum is {}\n", .{sum});

    return sum == num;
}
