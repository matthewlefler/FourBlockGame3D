use std::time::Duration;

use bevy::{input::gamepad::GamepadEvent, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{board::Board, constants, gamemodes::parse::parse_gamemode_file};

mod parse;
mod create;

// {
//     "name":"Insert Name Here",
//     "players": [
//         "List",
//         "Of",
//         ...,
//         "Player Names"
//     ],
//     "scores": [ 
//         Score_1,
//         Score_2,
//         Score_3,
//         Score_4
//     ],
//     "levels_enabled": false,
//     "BoardSettings": {
//         "width": i32,
//         "height": i32,
//    
//         "piece_queue_len": i32,
//         "piece_queue_spawn_point": [0, 0, 0],
//         "piece_queue_position": [0, 0, 0],
//    
//         "piece_storage_enabled": bool,
//    
//         "initial_time_between_piece_move_down": f64,
//         "minimum_time_between_piece_move_down": f64,
//         "time_between_piece_move_down_amount_decrease_by_piece": f64,
//    
//         "active_piece_place_time": f64
//     }
// }
#[derive(Debug, Component, Resource, Serialize, Deserialize)]
pub struct Gamemode {
    name: String,
    players: Vec<String>,
    scores: [i32; 4],
    levels_enabled: bool,
    board_settings: BoardSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoardSettings {
    width: i32,
    height: i32,

    piece_spawn_point: IVec2,

    piece_queue_len: usize,
    piece_queue_position: Vec3,

    piece_storage_enabled: bool,
    piece_storage_position: Vec3,

    initial_time_between_piece_move_down: f64,
    minimum_time_between_piece_move_down: f64,
    time_between_piece_move_down_amount_decrease_by_piece: f64,
    active_piece_place_time: f64,
}

impl From<&BoardSettings> for Board {   
    fn from(value: &BoardSettings) -> Self {
        Board::new(
            value.width, 
            value.height, 
            value.piece_queue_len, 
            value.piece_queue_position,
            value.piece_spawn_point, 
            Duration::from_secs_f64(value.active_piece_place_time), 
            Duration::from_secs_f64(value.initial_time_between_piece_move_down),
            value.piece_storage_enabled,
            value.piece_storage_position,
        )
    }
}

#[derive(Debug, Resource)]
pub struct GamemodesTemplates(Vec<Gamemode>);

// plugin
#[derive(Debug)]
pub struct GamemodesPlugin;

impl Plugin for GamemodesPlugin {
    fn build(&self, app: &mut App) {
        let gamemodes = GamemodesTemplates(parse_gamemode_file(constants::GAMEMODES_FILE));
        app.insert_resource(gamemodes);
    }
}