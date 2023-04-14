const std = @import("std");

pub const QueenError = error{
    InitializationFailure,
};

pub const Queen = struct {
    row: i8,
    col: i8,

    pub fn init(row: i8, col: i8) QueenError!Queen {
        if (row < 0 or col < 0 or row > 7 or col > 7) {
            return QueenError.InitializationFailure;
        }
        return Queen{ .row = row, .col = col };
    }

    pub fn canAttack(self: Queen, other: Queen) QueenError!bool {
        if (self.col == other.col or self.row == other.row) {
            return true;
        }
        const diffCol = std.math.absCast(self.col - other.col);
        const diffRow = std.math.absCast(self.row - other.row);
        if (diffCol == diffRow) {
            return true;
        }
        return false;
    }
};
