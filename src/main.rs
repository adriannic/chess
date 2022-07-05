use bevy::{prelude::*, window::WindowMode};

// Constants

const WINDOW_SIZE: f32 = 400.0;
const BOARD_SIZE: u32 = 8;
const TILE_SIZE: f32 = WINDOW_SIZE / BOARD_SIZE as f32;

#[derive(Clone, Copy)]
enum Roles {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq)]
enum Teams {
    White,
    Black,
}

#[derive(Clone, Copy)]
struct Piece {
    role: Roles,
    team: Teams,
}

impl Piece {
    fn pawn(team: Teams) -> Piece {
        Piece {
            role: Roles::Pawn,
            team,
        }
    }
    fn rook(team: Teams) -> Piece {
        Piece {
            role: Roles::Rook,
            team,
        }
    }
    fn knight(team: Teams) -> Piece {
        Piece {
            role: Roles::Knight,
            team,
        }
    }
    fn bishop(team: Teams) -> Piece {
        Piece {
            role: Roles::Bishop,
            team,
        }
    }
    fn queen(team: Teams) -> Piece {
        Piece {
            role: Roles::Queen,
            team,
        }
    }
    fn king(team: Teams) -> Piece {
        Piece {
            role: Roles::King,
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
                Some(Piece::queen(Teams::White)),
                Some(Piece::king(Teams::White)),
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
                Some(Piece::queen(Teams::Black)),
                Some(Piece::king(Teams::Black)),
                Some(Piece::bishop(Teams::Black)),
                Some(Piece::knight(Teams::Black)),
                Some(Piece::rook(Teams::Black)),
            ],
        ])
    }
}

struct Selected(Option<(u32, u32)>);

struct Turn(Teams);

impl Turn {
    fn toggle(&mut self) {
        self.0 = match self.0 {
            Teams::White => Teams::Black,
            Teams::Black => Teams::White,
        }
    }
}

#[derive(Component)]
struct RemovalTag {}

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
        .insert_resource(Selected(None))
        .insert_resource(Turn(Teams::White))
        .add_startup_system(setup)
        .add_system(draw_pieces)
        .add_system(move_piece)
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
                transform: Transform::from_xyz(board_to_screen(i), board_to_screen(j), 1.0),
                ..default()
            });
        }
    }
}

// Systems

fn draw_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    selected: Res<Selected>,
    pieces: Query<Entity, With<RemovalTag>>,
) {
    for entity in pieces.iter() {
        commands.entity(entity).despawn();
    }

    if let Some((x, y)) = selected.0 {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 1.0, 0.0, 0.25),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(board_to_screen(x), board_to_screen(y), 1.5),
                ..default()
            })
            .insert(RemovalTag {});
    }

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            match board.0[y as usize][x as usize] {
                None => continue,
                Some(piece) => {
                    commands
                        .spawn_bundle(SpriteBundle {
                            texture: asset_server.load(get_path(piece).as_str()),
                            transform: Transform::from_xyz(
                                board_to_screen(x),
                                board_to_screen(y),
                                2.0,
                            ),
                            ..default()
                        })
                        .insert(RemovalTag {});
                }
            }
        }
    }
}

fn move_piece(
    mut board: ResMut<Board>,
    mut selected: ResMut<Selected>,
    mut turn: ResMut<Turn>,
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
) {
    let (x, y) = match windows
        .get_primary()
        .expect("No primary window")
        .cursor_position()
    {
        None => return,
        Some(pos) => cursor_to_board(pos),
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        match selected.0 {
            None => match board.0[y as usize][x as usize] {
                None => return,
                Some(piece) => {
                    if piece.team == turn.0 {
                        selected.0 = Some((x, y));
                    }
                }
            },
            Some(pos) => {
                board.0[y as usize][x as usize] = board.0[pos.1 as usize][pos.0 as usize];
                board.0[pos.1 as usize][pos.0 as usize] = None;
                selected.0 = None;
                turn.toggle();
            }
        }
    }
}

// Auxiliary functions

fn board_to_screen(pos: u32) -> f32 {
    (WINDOW_SIZE / -2.0) + (pos as f32 + 0.5) * TILE_SIZE
}

fn screen_to_board(pos: f32) -> u32 {
    (pos / TILE_SIZE) as u32
}

fn cursor_to_board(pos: Vec2) -> (u32, u32) {
    (screen_to_board(pos.x), screen_to_board(pos.y))
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
        Roles::Pawn => "pawn",
        Roles::Rook => "rook",
        Roles::Knight => "knight",
        Roles::Bishop => "bishop",
        Roles::Queen => "queen",
        Roles::King => "king",
    };
    format!("sprites/{}_{}.png", role, team)
}
