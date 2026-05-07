use bevy::{prelude::*};
use serde::{Deserialize, Serialize};

const PIECE_FILE: &str = "./src/data/pieces.json";

#[derive(Serialize, Deserialize, Component, Debug)]
pub struct Piece {
    pub name: String,
    pub blocks: Vec<Position>
}


impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.blocks)
    }
}

#[derive(Serialize, Deserialize, Component, Debug)]
pub struct Position {
    pub x: i16,
    pub y: i16
}

#[derive(Resource)]
pub struct PieceTemplates(pub Vec<Piece>);

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        let piece_templates: Vec<Piece> =
            serde_json::from_reader(std::fs::File::open(PIECE_FILE).expect("unable to open PIECE_FILE")).expect("unable to parse PIECE_FILE");
        app.insert_resource(PieceTemplates(piece_templates));
    }
}
