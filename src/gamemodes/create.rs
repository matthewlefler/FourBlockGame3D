use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::math::UVec2;
use bevy::{ecs::system::Commands};

use crate::board::Board;
use crate::input::InputMapping;

#[derive(Component, Debug)]
pub struct Player {
    player: usize
}

pub fn start_gamemode(
    gamemode: super::Gamemode,
    commands: &mut Commands,
    window: bevy::ecs::system::Single<&bevy::window::Window>,
) {
    // enable board systems,
    // TODO: move camera?
    // TODO: start animations?
    let window_size = window.physical_size();
    
    let total_number_of_players = gamemode.players.len() as u32;
    for (i, player) in gamemode.players.iter().enumerate() {
        commands.spawn((
            Player { player: i },

            Board::from(&gamemode.board_settings),
            InputMapping::default(),

            (
                bevy::camera::Camera3d::default(),
                bevy::camera::Camera {
                    viewport: Some(bevy::camera::Viewport {
                        physical_position: UVec2::new((window_size.x / total_number_of_players) * i as u32, 0),
                        physical_size: UVec2::new(window_size.x / total_number_of_players, window_size.y),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                bevy::camera::Projection::from(bevy::camera::PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..bevy::utils::default()
                }),
            ),
        ));
    }
}

pub fn cleanup_games_system (
    commands: &mut Commands,
    entities: Query<Entity, With<Player>>
) {
    for entity in entities {
        commands.entity(entity).despawn();
    }
}