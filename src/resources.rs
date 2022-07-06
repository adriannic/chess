use crate::types::*;
use crate::constants::*;

/// Represents the chess board
pub struct Board(pub [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    pub fn get(&self, pos: &Pos) -> Option<Piece> {
        self.0[pos.y][pos.x]
    }
    pub fn set(&mut self, pos: &Pos, piece: Option<Piece>) {
        self.0[pos.y][pos.x] = piece;
    }
    pub fn set_with_pos(&mut self, pos: &Pos, other: &Pos) {
        self.0[pos.y][pos.x] = self.get(other);
    }
}

impl Default for Board {
    fn default() -> Self {
        Board([
            [
                Some(Piece::rook(Teams::White)),
                Some(Piece::knight(Teams::White)),
                Some(Piece::bishop(Teams::White)),
                Some(Piece::queen(Teams::White)),
                Some(Piece::king(Teams::White)),
                Some(Piece::bishop(Teams::White)),
                Some(Piece::knight(Teams::White)),
                Some(Piece::rook(Teams::White)),
            ],
            [Some(Piece::pawn(Teams::White)); BOARD_SIZE],
            [None; BOARD_SIZE],
            [None; BOARD_SIZE],
            [None; BOARD_SIZE],
            [None; BOARD_SIZE],
            [Some(Piece::pawn(Teams::Black)); BOARD_SIZE],
            [
                Some(Piece::rook(Teams::Black)),
                Some(Piece::knight(Teams::Black)),
                Some(Piece::bishop(Teams::Black)),
                Some(Piece::queen(Teams::Black)),
                Some(Piece::king(Teams::Black)),
                Some(Piece::bishop(Teams::Black)),
                Some(Piece::knight(Teams::Black)),
                Some(Piece::rook(Teams::Black)),
            ],
        ])
    }
}

pub struct Selected(pub Option<Pos>);

pub struct Turn(pub Teams);

impl Turn {
    pub fn toggle(&mut self) {
        self.0 = match self.0 {
            Teams::White => Teams::Black,
            Teams::Black => Teams::White,
        }
    }
}