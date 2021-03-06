use crate::constants::*;
use crate::resources::*;
use crate::tags::*;
use crate::types::*;
use crate::util::*;
use bevy::prelude::*;

// Startup systems

pub fn setup(mut commands: Commands) {
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
                transform: Transform::from_xyz(
                    board_to_screen(i as i32),
                    board_to_screen(j as i32),
                    1.0,
                ),
                ..default()
            });
        }
    }
}

// Systems

pub fn draw_pieces(
    mut commands: Commands,
    check: Res<Check>,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    selected: Res<Selected>,
    pieces: Query<Entity, With<RemovalTag>>,
) {
    for entity in pieces.iter() {
        commands.entity(entity).despawn();
    }

    if let Some(pos) = selected.0 {
        let mut positions = get_valid_moves(&*board, &pos);
        positions.push(pos);
        for pos in positions.iter() {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, 0.25),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board_to_screen(pos.x),
                        board_to_screen(pos.y),
                        1.5,
                    ),
                    ..default()
                })
                .insert(RemovalTag {});
        }
    }

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if let Some(piece) = board.0[y][x] {
                if let Some(check_team) = check.0 {
                    if piece.role == Roles::King && check_team == piece.team {
                        commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 0.0, 0.0, 0.75),
                                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    board_to_screen(x as i32),
                                    board_to_screen(y as i32),
                                    1.5,
                                ),
                                ..default()
                            })
                            .insert(RemovalTag {});
                    }
                }
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.load(get_path(piece).as_str()),
                        transform: Transform::from_xyz(
                            board_to_screen(x as i32),
                            board_to_screen(y as i32),
                            2.0,
                        ),
                        ..default()
                    })
                    .insert(RemovalTag {});
            }
        }
    }
}

pub fn move_piece(
    mut board: ResMut<Board>,
    mut selected: ResMut<Selected>,
    mut turn: ResMut<Turn>,
    mut check: ResMut<Check>,
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
) {
    update_check_state(&board, &mut check);

    let cursor_pos = match windows
        .get_primary()
        .expect("No primary window")
        .cursor_position()
    {
        Some(pos) => cursor_to_board(pos),
        None => return,
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(selected_pos) = selected.0 {
            if selected_pos == cursor_pos {
                selected.0 = None;
            } else {
                if !get_valid_moves(&*board, &selected_pos).contains(&cursor_pos) {
                    return;
                }

                let old_piece = board.get(&cursor_pos);
                board.set_with_pos(&cursor_pos, &selected_pos);
                board.set(&selected_pos, None);
                let old_check_state = check.0;
                update_check_state(&board, &mut check);
                if let Some(team) = check.0 {
                    if team == turn.0 {
                        board.set_with_pos(&selected_pos, &cursor_pos);
                        board.set(&cursor_pos, old_piece);
                        check.0 = old_check_state;
                        return;
                    }
                }
                selected.0 = None;
                board.get(&cursor_pos).unwrap().has_moved = true;
                turn.toggle();
            }
        } else {
            if let Some(piece) = board.get(&cursor_pos) {
                if piece.team == turn.0 {
                    selected.0 = Some(cursor_pos);
                }
            }
        }
    }
}
