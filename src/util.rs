use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;

pub fn board_to_screen(pos: usize) -> f32 {
    (WINDOW_SIZE / -2.0) + (pos as f32 + 0.5) * TILE_SIZE
}

pub fn screen_to_board(pos: f32) -> usize {
    (pos / TILE_SIZE) as usize
}

pub fn cursor_to_board(pos: Vec2) -> Pos {
    Pos {
        x: screen_to_board(pos.x),
        y: screen_to_board(pos.y),
    }
}

pub fn is_even(n: usize) -> bool {
    n & 1 == 0
}

pub fn get_path(piece: Piece) -> String {
    let team = match piece.team {
        Teams::Black => "black",
        Teams::White => "white",
    };
    let role = match piece.role {
        Roles::Pawn => "pawn",
        Roles::Rook => "rook",
        Roles::Knight => "knight",
        Roles::Bishop => "bishop",
        Roles::Queen => "queen",
        Roles::King => "king",
    };
    format!("sprites/{}_{}.png", role, team)
}