use bevy::prelude::*;

use crate::piece::Position;

#[derive(Component, Debug)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub state: Vec<bool>,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            state: Vec::with_capacity((width * height) as usize),
        }
    }
}

pub fn is_cell_occupied(board: &Board, x: i32, y: i32) -> bool {
    board.state[(x + y * board.width) as usize]
}

pub fn place_blocks(board: &mut Board, blocks: &Vec<Position>) {
    for block in blocks {
        board.state[(block.x + block.y * board.width) as usize] = true;
    }
}

