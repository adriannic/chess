use crate::constants::*;
use crate::*;

pub fn board_to_screen(pos: i32) -> f32 {
    (WINDOW_SIZE / -2.0) + (pos as f32 + 0.5) * TILE_SIZE
}

pub fn screen_to_board(pos: f32) -> i32 {
    (pos / TILE_SIZE) as i32
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

pub fn get_valid_moves(board: &Board, piece_pos: &Pos) -> Vec<Pos> {
    let mut result = vec![];
    let piece = board.get(&piece_pos).unwrap();

    let direction = if piece.team == Teams::White { 1 } else { -1 };

    match piece.role {
        Roles::Pawn => {
            // move 1
            let candidate = piece_pos.add(0, direction);
            if let None = board.get(&candidate) {
                result.push(candidate);
            }
            // diagonals
            for i in (-1..=1).step_by(2) {
                let diagonal = candidate.add(i, 0);
                if is_pos_team(&board, &diagonal, &piece.team.toggle()) {
                    result.push(diagonal);
                }
            }
            // move 2
            let candidate = piece_pos.add(0, direction * 2);
            let can_move_two = if piece.team == Teams::White {
                piece_pos.y == 1
            } else {
                piece_pos.y == 6
            };
            if can_move_two
                && result.contains(&piece_pos.add(0, direction))
                && board.get(&candidate) == None
            {
                result.push(candidate);
            }
        }
        Roles::Rook => {
            let directions = Pos { x: 0, y: 1 };
            let directions = [
                directions,
                directions.mul(-1, -1),
                directions.inverse(),
                directions.inverse().mul(-1, -1),
            ];
            for direction in directions.iter() {
                for i in 1..8 {
                    let candidate = piece_pos.add_pos(&direction.mul(i, i));
                    if candidate.out_of_bounds() || is_pos_team(&board, &candidate, &piece.team) {
                        break;
                    }
                    result.push(candidate);
                    if is_pos_team(&board, &candidate, &piece.team.toggle()) {
                        break;
                    }
                }
            }
        }
        Roles::Knight => {
            let directions = Pos { x: 2, y: 1 };
            let directions = [
                directions,
                directions.mul(-1, -1),
                directions.mul(-1, 1),
                directions.mul(1, -1),
                directions.inverse(),
                directions.inverse().mul(-1, -1),
                directions.inverse().mul(-1, 1),
                directions.inverse().mul(1, -1),
            ];
            for direction in directions.iter() {
                let candidate = piece_pos.add_pos(direction);
                if candidate.out_of_bounds() || is_pos_team(&board, &candidate, &piece.team) {
                    continue;
                }
                result.push(candidate);
            }
        }
        Roles::Bishop => {
            let directions = Pos { x: 1, y: 1 };
            let directions = [
                directions,
                directions.mul(-1, -1),
                directions.mul(1, -1),
                directions.mul(-1, 1),
            ];
            for direction in directions.iter() {
                for i in 1..8 {
                    let candidate = piece_pos.add_pos(&direction.mul(i, i));
                    if candidate.out_of_bounds() || is_pos_team(&board, &candidate, &piece.team) {
                        break;
                    }
                    result.push(candidate);
                    if is_pos_team(&board, &candidate, &piece.team.toggle()) {
                        break;
                    }
                }
            }
        }
        Roles::Queen => {
            let mut directions = vec![];
            for i in -1..=1 {
                for j in -1..=1 {
                    if i != 0 || j != 0 {
                        directions.push(Pos { x: i, y: j });
                    }
                }
            }
            for direction in directions.iter() {
                for i in 1..8 {
                    let candidate = piece_pos.add_pos(&direction.mul(i, i));
                    if candidate.out_of_bounds() || is_pos_team(&board, &candidate, &piece.team) {
                        break;
                    }
                    result.push(candidate);
                    if is_pos_team(&board, &candidate, &piece.team.toggle()) {
                        break;
                    }
                }
            }
        }
        Roles::King => {
            for i in -1..=1 {
                for j in -1..=1 {
                    let candidate = piece_pos.add(i, j);
                    if (i != 0 || j != 0)
                        && !candidate.out_of_bounds()
                        && !is_pos_team(&board, &candidate, &piece.team)
                    {
                        result.push(candidate);
                    }
                }
            }
            // Castling
            for i in (-2..=2).step_by(4) {
                let candidate = piece_pos.add(i, 0);
                let rook_pos = if i == -2 {
                    Pos {
                        x: 0,
                        y: piece_pos.y,
                    }
                } else {
                    Pos {
                        x: 7,
                        y: piece_pos.y,
                    }
                };
                if !candidate.out_of_bounds()
                    && !board.get(&piece_pos).unwrap().has_moved
                    && board.get(&rook_pos) != None
                    && board.get(&rook_pos).unwrap().role == Roles::Rook
                    && !board.get(&rook_pos).unwrap().has_moved
                    && board.get(&candidate) == None
                    && board.get(&piece_pos.add(i / 2, 0)) == None
                {}
            }
        }
    }

    result
}

pub fn is_pos_team(board: &Board, pos: &Pos, team: &Teams) -> bool {
    if let Some(piece) = board.get(&pos) {
        piece.team == *team
    } else {
        false
    }
}

pub fn get_attacked_cells(board: &Board, team: &Teams) -> Vec<Pos> {
    let mut possible_attacks = vec![];
    for y in 0..8 {
        for x in 0..8 {
            let pos = Pos { x, y };
            if let Some(piece) = board.get(&pos) {
                if piece.team == *team {
                    possible_attacks.push(get_valid_moves(&board, &pos));
                }
            }
        }
    }
    let mut possible_attacks = possible_attacks.concat();
    possible_attacks.sort_unstable();
    possible_attacks.dedup();
    possible_attacks
}

pub fn update_check_state(board: &Board, check: &mut Check) {
    check.0 = None;
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if let Some(piece) = board.0[y][x] {
                if piece.role == Roles::King
                    && get_attacked_cells(&board, &piece.team.toggle()).contains(&Pos {
                        x: x as i32,
                        y: y as i32,
                    })
                {
                    check.0 = Some(piece.team);
                }
            }
        }
    }
}
