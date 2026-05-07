use bevy::{prelude::*};
use serde::{Deserialize, Serialize};

const PIECE_FILE: &str = "./src/data/pieces.json";

#[derive(Resource)]
pub struct PieceTemplates(pub Vec<Piece>);

#[derive(Serialize, Deserialize, Component, Debug)]
pub struct Piece {
    pub name: String,
    pub blocks: Vec<Position>,
    pub center_point: Position,
    pub kick_offsets: KickOffsets,
}


impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.blocks)
    }
}

#[derive(Serialize, Deserialize, Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RotationKicks {
    pub clockwise: Vec<Position>,
    pub counter_clockwise: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KickOffsets {
    pub up: RotationKicks,
    pub right: RotationKicks,
    pub down: RotationKicks,
    pub left: RotationKicks,
}

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        let piece_templates: Vec<Piece> =
            serde_json::from_reader(std::fs::File::open(PIECE_FILE).expect("unable to open PIECE_FILE")).expect("unable to parse PIECE_FILE");
        app.insert_resource(PieceTemplates(piece_templates));
    }
}

