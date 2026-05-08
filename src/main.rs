use std::f32::consts::PI;

use bevy::{
    prelude::*,
    input::mouse::*,
};

mod piece;
use piece::*;

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
        .add_plugins(PiecePlugin)
        .add_systems(Startup, (
            setup,
            fps_camera::spawn_player_camera
        ))
        .add_systems(Update, (
            fps_camera::move_player
        ))
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    piece_templates: Res<PieceTemplates>,
) {
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 10.0, -8.0),
    ));

    let mut z_offset: f32 = 0.0;
    for piece in piece_templates.0.iter() {

        let mut x_offset: f32 = 0.0;
        for rotation in 0..4 {
            let angle = PI * 0.5 * rotation as f32;

            let rot = Quat::from_rotation_z(angle);

            for position in &piece.blocks {

                // local position relative to center
                let local = Vec3::new(
                    position.x as f32 - piece.center_point.x as f32,
                    position.y as f32 - piece.center_point.y as f32,
                    0.0,
                );

                // rotate the position
                let rotated = rot * local;

                // move back from center
                let final_pos = rotated
                    + Vec3::new(
                        piece.center_point.x as f32 + x_offset,
                        piece.center_point.y as f32,
                        z_offset,
                    );

                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
                    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                    Transform::from_translation(final_pos),
                ));

            }

            // center point
            let mut material = StandardMaterial::from_color(Color::srgb_u8(255, 255, 255));
            material.unlit = true;
            commands.spawn((
                Mesh3d(meshes.add(Segment3d::new(Vec3::new(0.0,0.0,-1.3), Vec3::new(0.0,0.0,1.3)))),
                MeshMaterial3d(materials.add(material)),
                Transform::from_translation(Vec3::new(
                    piece.center_point.x as f32 + x_offset,
                    piece.center_point.y as f32,
                    z_offset,
                )),
            ));

            x_offset += 10.0;
        }
        z_offset -= 4.0;
    }
}
