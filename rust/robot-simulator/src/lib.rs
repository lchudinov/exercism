// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.d {
            Direction::East => Robot {
                d: Direction::South,
                ..self
            },
            Direction::North => Robot {
                d: Direction::East,
                ..self
            },
            Direction::South => Robot {
                d: Direction::West,
                ..self
            },
            Direction::West => Robot {
                d: Direction::North,
                ..self
            },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.d {
            Direction::East => Robot {
                d: Direction::North,
                ..self
            },
            Direction::North => Robot {
                d: Direction::West,
                ..self
            },
            Direction::South => Robot {
                d: Direction::East,
                ..self
            },
            Direction::West => Robot {
                d: Direction::South,
                ..self
            },
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::East => Robot {
                x: self.x + 1,
                ..self
            },
            Direction::North => Robot {
                y: self.y + 1,
                ..self
            },
            Direction::South => Robot {
                y: self.y - 1,
                ..self
            },
            Direction::West => Robot {
                x: self.x - 1,
                ..self
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, ch| match ch {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'A' => robot.advance(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
