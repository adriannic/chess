use crate::constants::*;
use crate::types::*;

/// Represents the chess board
pub struct Board(pub [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    pub fn get(&self, pos: &Pos) -> Option<Piece> {
        if pos.out_of_bounds() {
            None
        } else {
            self.0[pos.y as usize][pos.x as usize]
        }
    }
    pub fn set(&mut self, pos: &Pos, piece: Option<Piece>) {
        if !pos.out_of_bounds() {
            self.0[pos.y as usize][pos.x as usize] = piece;
        }
    }
    pub fn set_with_pos(&mut self, pos: &Pos, other: &Pos) {
        if !pos.out_of_bounds() {
            self.0[pos.y as usize][pos.x as usize] = self.get(other);
        }
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
        self.0 = self.0.toggle();
    }
}

pub struct Check(pub Option<Teams>);