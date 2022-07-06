// Chess piece

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
#[derive(Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    /// Returns the Manhattan distance between two positions.
    pub fn diff(&self, other: &Pos) -> (i32, i32) {
        (
            other.x as i32 - self.x as i32,
            other.y as i32 - self.y as i32,
        )
    }
}