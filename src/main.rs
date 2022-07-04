use bevy::{prelude::*, window::WindowMode};

// Constants

const WINDOW_SIZE: f32 = 400.0;
const BOARD_SIZE: u32 = 8;
const TILE_SIZE: f32 = WINDOW_SIZE / BOARD_SIZE as f32;

// Components

#[derive(Component)]
enum Pieces {
    Empty,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Component)]
enum Teams {
    White,
    Black,
}

// Bundles

#[derive(Bundle)]
struct Piece {
    piece: Pieces,
    team: Teams,
    #[bundle]
    sprite: SpriteBundle,
}

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
        .add_startup_system(setup)
        .run();
}

// Startup systems

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if is_even(i + j) {
                continue;
            }
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
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

// Auxiliary functions

fn coords(pos: u32) -> f32 {
    (WINDOW_SIZE / -2.0) + (pos as f32 + 0.5) * TILE_SIZE
}

fn is_even(n: u32) -> bool {
    n & 1 == 0
}
