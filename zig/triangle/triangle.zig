const std = @import("std");
const math = std.math;
const f64_epsilon = math.f64_epsilon;

pub const TriangleError = error{Invalid};

pub const Triangle = struct {
    a: f64,
    b: f64,
    c: f64,

    pub fn init(a: f64, b: f64, c: f64) TriangleError!Triangle {
        if (a + b - c > f64_epsilon and b + c - a > f64_epsilon and a + c - b > f64_epsilon) {
            return Triangle{ .a = a, .b = b, .c = c };
        }
        return TriangleError.Invalid;
    }

    pub fn isEquilateral(self: Triangle) bool {
        return self.numberOfEqualSides() == 3;
    }

    pub fn isIsosceles(self: Triangle) bool {
        return self.numberOfEqualSides() == 2;
    }

    pub fn isScalene(self: Triangle) bool {
        return self.numberOfEqualSides() == 0;
    }

    fn numberOfEqualSides(self: Triangle) usize {
        const abEq = math.fabs(self.a - self.b) < f64_epsilon;
        const acEq = math.fabs(self.a - self.c) < f64_epsilon;
        const bcEq = math.fabs(self.b - self.c) < f64_epsilon;
        if (abEq and acEq and bcEq) {
            return 3;
        }
        if (abEq or acEq or bcEq) {
            return 2;
        }
        return 0;
    }
};
