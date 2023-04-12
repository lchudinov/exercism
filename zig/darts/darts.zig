pub const Coordinate = struct {
    x: f32,
    y: f32,

    pub fn init(x_coord: f32, y_coord: f32) Coordinate {
        return Coordinate{ .x = x_coord, .y = y_coord };
    }

    pub fn score(self: Coordinate) usize {
        const distance = self.x * self.x + self.y * self.y;
        if (distance > 100.0) {
            return 0;
        }
        if (distance > 25.0) {
            return 1;
        }
        if (distance > 1.0) {
            return 5;
        }
        return 10;
    }
};
