use bevy::{input::keyboard::KeyCode, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Component, Deserialize, Serialize, Debug)]
pub struct InputMapping {
    translate_left: Vec<KeyCode>,
    translate_right: Vec<KeyCode>,
    rotate_clockwise: Vec<KeyCode>,
    rotate_counter_clockwise: Vec<KeyCode>,

    slam_down: Vec<KeyCode>,
    stash_piece: Vec<KeyCode>,

    pause: Vec<KeyCode>,

}

impl Default for InputMapping {
    fn default() -> Self {
        Self { 
            translate_left: vec![KeyCode::KeyJ], 
            translate_right: vec![KeyCode::KeyL], 
            rotate_clockwise: vec![KeyCode::KeyI], 
            rotate_counter_clockwise: vec![KeyCode::KeyK], 

            slam_down: vec![KeyCode::Space],
            stash_piece: vec![KeyCode::KeyH],

            pause: vec![KeyCode::Escape],
        }
    }
}
