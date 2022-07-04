use bevy::{prelude::*, window::WindowMode};

// Constants

const WINDOW_SIZE: f32 = 400.0;
const BOARD_SIZE: u32 = 8;
const TILE_SIZE: f32 = WINDOW_SIZE / BOARD_SIZE as f32;

#[derive(Clone, Copy)]
enum PieceRoles {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
enum Teams {
    White,
    Black,
}

#[derive(Clone, Copy)]
struct Piece {
    role: PieceRoles,
    team: Teams,
}

impl Piece {
    fn pawn(team: Teams) -> Piece {
        Piece {
            role: PieceRoles::Pawn,
            team,
        }
    }
    fn rook(team: Teams) -> Piece {
        Piece {
            role: PieceRoles::Rook,
            team,
        }
    }
    fn knight(team: Teams) -> Piece {
        Piece {
            role: PieceRoles::Knight,
            team,
        }
    }
    fn bishop(team: Teams) -> Piece {
        Piece {
            role: PieceRoles::Bishop,
            team,
        }
    }
    fn queen(team: Teams) -> Piece {
        Piece {
            role: PieceRoles::Queen,
            team,
        }
    }
    fn king(team: Teams) -> Piece {
        Piece {
            role: PieceRoles::King,
            team,
        }
    }
}

struct Board([[Option<Piece>; BOARD_SIZE as usize]; BOARD_SIZE as usize]);

impl Default for Board {
    fn default() -> Self {
        Board([
            [
                Some(Piece::rook(Teams::White)),
                Some(Piece::knight(Teams::White)),
                Some(Piece::bishop(Teams::White)),
                Some(Piece::king(Teams::White)),
                Some(Piece::queen(Teams::White)),
                Some(Piece::bishop(Teams::White)),
                Some(Piece::knight(Teams::White)),
                Some(Piece::rook(Teams::White)),
            ],
            [Some(Piece::pawn(Teams::White)); BOARD_SIZE as usize],
            [None; BOARD_SIZE as usize],
            [None; BOARD_SIZE as usize],
            [None; BOARD_SIZE as usize],
            [None; BOARD_SIZE as usize],
            [Some(Piece::pawn(Teams::Black)); BOARD_SIZE as usize],
            [
                Some(Piece::rook(Teams::Black)),
                Some(Piece::knight(Teams::Black)),
                Some(Piece::bishop(Teams::Black)),
                Some(Piece::king(Teams::Black)),
                Some(Piece::queen(Teams::Black)),
                Some(Piece::bishop(Teams::Black)),
                Some(Piece::knight(Teams::Black)),
                Some(Piece::rook(Teams::Black)),
            ],
        ])
    }
}

#[derive(Component)]
struct PieceTag {}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            mode: WindowMode::Windowed,
            resizable: false,
            title: "Chess".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Board::default())
        .add_startup_system(setup)
        .add_system(draw_pieces)
        .run();
}

// Startup systems

fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Draw board

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if is_even(i + j) {
                continue;
            }
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.6, 0.6, 0.6),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(coords(i), coords(j), 1.0),
                ..default()
            });
        }
    }
}

// Systems

fn draw_pieces(mut commands: Commands, asset_server: Res<AssetServer>, board: Res<Board>, pieces: Query<Entity, With<PieceTag>>) {
    for entity in pieces.iter() {
        commands.entity(entity).despawn();
    }
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            match board.0[y as usize][x as usize] {
                None => continue,
                Some(piece) => {
                    commands
                        .spawn_bundle(SpriteBundle {
                            texture: asset_server.load(get_path(piece).as_str()),
                            transform: Transform::from_xyz(coords(x), coords(y), 2.0),
                            ..default()
                        })
                        .insert(PieceTag {});
                }
            }
        }
    }
}

// Auxiliary functions

fn coords(pos: u32) -> f32 {
    (WINDOW_SIZE / -2.0) + (pos as f32 + 0.5) * TILE_SIZE
}

fn is_even(n: u32) -> bool {
    n & 1 == 0
}

fn get_path(piece: Piece) -> String {
    let team = match piece.team {
        Teams::Black => "black",
        Teams::White => "white",
    };
    let role = match piece.role {
        PieceRoles::Pawn => "pawn",
        PieceRoles::Rook => "rook",
        PieceRoles::Knight => "knight",
        PieceRoles::Bishop => "bishop",
        PieceRoles::Queen => "queen",
        PieceRoles::King => "king",
    };
    format!("sprites/{}_{}.png", role, team)
}
