use bevy::prelude::*;

mod piece;
mod board;
mod debug;
mod fps_camera;

use piece::*;


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "FourBlockGame3D".into(),
                    // resolution: (600, 600).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins((
            PiecePlugin,
        ))
        .add_systems(Startup, (
            debug_setup,
            fps_camera::spawn_player_camera,
            board::setup,
            board::board_new_piece_system
        ).chain())
        .add_systems(Update, (
            fps_camera::move_player,
            fps_camera::grab_mouse,
            (
                board::move_and_rotate_piece_system,
                board::move_piece_down_system,
                board::place_piece_system,
            ).chain(),
            debug::debug_pos_text_system,
        ))
        .run();
}

fn debug_setup(
    mut commands: Commands,
) {
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
