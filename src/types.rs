// Chess piece

use crate::constants::BOARD_SIZE;

/// Enumerates all the possible roles of a chess piece
#[derive(Clone, Copy, PartialEq)]
pub enum Roles {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// Enumerates all the possible teams of a chess piece
#[derive(Clone, Copy, PartialEq)]
pub enum Teams {
    White,
    Black,
}

impl Teams {
    pub fn toggle(&self) -> Teams {
        match self {
            Teams::White => Teams::Black,
            Teams::Black => Teams::White,
        }
    }
}

/// Represents a chess piece
#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub role: Roles,
    pub team: Teams,
}

impl Piece {
    /// Creates a new pawn of the given team
    pub fn pawn(team: Teams) -> Piece {
        Piece {
            role: Roles::Pawn,
            team,
        }
    }
    /// Creates a new rook of the given team
    pub fn rook(team: Teams) -> Piece {
        Piece {
            role: Roles::Rook,
            team,
        }
    }
    /// Creates a new knight of the given team
    pub fn knight(team: Teams) -> Piece {
        Piece {
            role: Roles::Knight,
            team,
        }
    }
    /// Creates a new bishop of the given team
    pub fn bishop(team: Teams) -> Piece {
        Piece {
            role: Roles::Bishop,
            team,
        }
    }
    /// Creates a new queen of the given team
    pub fn queen(team: Teams) -> Piece {
        Piece {
            role: Roles::Queen,
            team,
        }
    }
    /// Creates a new king of the given team
    pub fn king(team: Teams) -> Piece {
        Piece {
            role: Roles::King,
            team,
        }
    }
}

// Mathematical types

/// Represents a board position
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn add(&self, x: i32, y: i32) -> Pos {
        Pos {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn add_pos(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn mul(&self, x: i32, y: i32) -> Pos {
        Pos {
            x: self.x * x,
            y: self.y * y,
        }
    }

    pub fn inverse(&self) -> Pos {
        Pos {
            x: self.y,
            y: self.x,
        }
    }

    pub fn out_of_bounds(&self) -> bool {
        self.x >= BOARD_SIZE as i32 || self.x < 0 || self.y >= BOARD_SIZE as i32 || self.y < 0
    }
}
