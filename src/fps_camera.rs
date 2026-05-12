use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::{CursorGrabMode, CursorOptions}};

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            // We use a component for them so that we can make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect` the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

#[derive(Debug, Component)]
pub struct WorldModelCamera;

pub fn spawn_player_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        CameraSensitivity::default(),
        Transform::from_xyz(5.0, 30.0, 30.0),
        Visibility::default(),
        children![
            (
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ),
        ]
    ));
}

pub fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    cursor_options: Single<&CursorOptions>,
) {
    if cursor_options.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }

    let mut movement = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) {
        movement -= 3.0 * transform.local_z() * time.delta_secs();
    }
    if input.pressed(KeyCode::KeyS)  {
        movement += 3.0 * transform.local_z() * time.delta_secs();
    }
    if input.pressed(KeyCode::KeyA) {
        movement -= 3.0 * transform.local_x() * time.delta_secs();
    }
    if input.pressed(KeyCode::KeyD) {
        movement += 3.0 * transform.local_x() * time.delta_secs();
    }
    // down
    if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::KeyQ) {
        movement -= 3.0 * transform.local_y() * time.delta_secs();
    }
    // up
    if input.pressed(KeyCode::Space) || input.pressed(KeyCode::KeyE) {
        movement += 3.0 * transform.local_y() * time.delta_secs();
    }

    transform.translation += movement;
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
pub fn grab_mouse(
    mut cursor_options: Single<&mut CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}
