#[derive(Debug)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || file < 0 || rank > 7 || file > 7 {
            return None;
        }
        Some(ChessPosition {
            rank: rank as u8,
            file: file as u8,
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank {
            return true;
        } else if self.position.file == other.position.file {
            return true;
        } else if self.position.rank.abs_diff(other.position.rank)
            == self.position.file.abs_diff(other.position.file)
        {
            return true;
        }
        return false;
    }
}
