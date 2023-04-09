pub const ChessboardError = error{IndexOutOfBounds};

pub fn square(index: usize) ChessboardError!u64 {
    if (index == 0 or index > 64) {
        return ChessboardError.IndexOutOfBounds;
    }
    const power = @intCast(u6, index - 1);
    return @shlExact(@as(u64, 1), power);
}

pub fn total() u64 {
    var sum: u64 = 0;
    var i: u32 = 0;
    while (i < 64) {
        const count = square(i + 1) catch unreachable;
        sum += count;
        i += 1;
    }
    return sum;
}
