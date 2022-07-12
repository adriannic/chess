use bevy::{prelude::*, window::WindowMode};
use constants::*;
use resources::*;
use systems::*;
use types::*;

mod constants;
mod resources;
mod systems;
mod tags;
mod types;
mod util;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            title: "Chess".to_string(),
            resizable: false,
            mode: WindowMode::Windowed,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Board::default())
        .insert_resource(Selected(None))
        .insert_resource(Turn(Teams::White))
        .insert_resource(Check(None))
        .add_startup_system(setup)
        .add_system(draw_pieces)
        .add_system(move_piece)
        .run();
}
