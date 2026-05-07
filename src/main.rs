use std::f32::consts::{FRAC_2_PI, PI};

use bevy::prelude::*;

mod piece;
use piece::*;

mod board;

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
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_camera)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    piece_templates: Res<PieceTemplates>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_xyz(-7.0, 20.0, 4.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

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
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                    Transform::from_translation(final_pos),
                ));
            }

            x_offset += 5.0;
        }
        z_offset -= 2.0;
    }
}

// Orbit system
fn orbit_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    const OFFSET: Vec3 = Vec3::new(8.0, 0.0, -8.0);
    const RADIUS: f32 = 40.0;
    let height = 20.0;
    let speed = std::f32::consts::PI / 8.0; // 22.5 degrees per second

    for mut transform in &mut query {
        // angle based on elapsed time
        let angle = time.elapsed_secs() * speed;

        // x and z position along a circle
        let x = RADIUS * angle.cos();
        let z = RADIUS * angle.sin();

        transform.translation = Vec3::new(x, height, z) + OFFSET;
        transform.look_at(Vec3::ZERO + OFFSET, Vec3::Y); // always look at center
    }
}