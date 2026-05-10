
use bevy::prelude::*;

mod piece;
use piece::*;

use crate::board::Board;

mod board;

mod fps_camera;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "FourBlockGame3D".into(),
                    resolution: (600, 600).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins((
            PiecePlugin,
        ))
        .add_systems(Startup, (
            setup,
            fps_camera::spawn_player_camera,
            board::setup,
            board::board_new_piece_system
        ).chain())
        .add_systems(Update, (
            fps_camera::move_player,
            board::move_and_rotate_piece_system,
        ))
        .run();
}

fn setup(
    mut commands: Commands
) {
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(8.0, 10.0, -8.0),
    ));
}
