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
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let mut z_position: f32 = 0.0;
    for piece in piece_templates.0.iter() {
        for position in &piece.blocks {
            commands.spawn( (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                Transform::from_xyz(position.x as f32, position.y as f32, z_position),
            ));
        }
        z_position -= 1.5;
    }
}

// Orbit system
fn orbit_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let radius = 20.0;
    let height = 10.0;
    let speed = std::f32::consts::PI / 8.0; // 22.5 degrees per second

    for mut transform in &mut query {
        // angle based on elapsed time
        let angle = time.elapsed_secs() * speed;

        // x and z position along a circle
        let x = radius * angle.cos();
        let z = radius * angle.sin();

        transform.translation = Vec3::new(x, height, z);
        transform.look_at(Vec3::ZERO, Vec3::Y); // always look at center
    }
}