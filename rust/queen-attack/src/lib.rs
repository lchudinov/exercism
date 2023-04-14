#[derive(Debug)]
pub struct ChessPosition {
    row: i32,
    col: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..=7).contains(&rank) || !(0..=7).contains(&file) {
            None
        } else {
            Some(ChessPosition{row: rank, col: file})
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.pos.col == other.pos.col || self.pos.row == other.pos.row {
            return true;
        }
        let diff_col = (self.pos.col - other.pos.col).abs();
        let diff_row = (self.pos.row - other.pos.row).abs();
        diff_col == diff_row
    }
}
