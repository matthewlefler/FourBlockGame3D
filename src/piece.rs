use bevy::{prelude::*};
use serde::{Deserialize, Serialize};

use crate::board::{Board, block_fits, piece_fits};
use crate::constants;
use crate::debug::{DebugPosText, spawn_debug_text};

#[derive(Resource)]
pub struct PieceTemplates(pub Vec<PieceTemplate>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PieceTemplate {
    pub name: String,
    blocks: Vec<IVec2>,
    pub center_point: IVec2,
    kick_offsets: KickOffsets,
}

#[derive(Component, Clone, Debug)]
pub struct Piece {
    pub template: PieceTemplate,
    blocks: Vec<IVec2>,
    position: IVec2,
    pub cubes: Vec<Entity>,
    pub facing: Facing,
}

impl Piece {
    pub fn move_to(&mut self, pos: IVec2) {
        self.position = pos
    }
}

#[derive(Component)]
pub struct Cube;

#[derive(Clone, Copy, Debug)]
pub enum Rotation {
    Clockwise,
    CounterClockwise
}

#[derive(Clone, Debug)]
pub enum Facing {
    Up,
    Right,
    Down,
    Left,
}

impl Facing {
    fn next(&self) -> Self {
        match self {
            Facing::Up =>    Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down =>  Facing::Left,
            Facing::Left =>  Facing::Up,
        }
    }

    fn previous(&self) -> Self {
        match self {
            Facing::Up =>    Facing::Left,
            Facing::Right => Facing::Up,
            Facing::Down =>  Facing::Right,
            Facing::Left =>  Facing::Down,
        }
    }
}

pub trait Rotate {
    fn rotate(&mut self, dir: Rotation);
}

impl Rotate for PieceTemplate {
    fn rotate(&mut self, dir: Rotation) {
        for block in &mut self.blocks {
            let x = block.x - self.center_point.x;
            let y = block.y - self.center_point.y;

            match dir {
                Rotation::Clockwise => {
                    block.x = self.center_point.x + y;
                    block.y = self.center_point.y - x;
                }
                Rotation::CounterClockwise => {
                    block.x = self.center_point.x - y;
                    block.y = self.center_point.y + x;
                }
            }
        }
    }
}

impl Rotate for Piece {
    fn rotate(&mut self, dir: Rotation) {
        match dir {
            Rotation::Clockwise => {
                self.facing = self.facing.next();
            }
            Rotation::CounterClockwise => {
                self.facing = self.facing.previous();
            }
        }

        for block in &mut self.blocks {
            let x = block.x - self.template.center_point.x;
            let y = block.y - self.template.center_point.y;

            match dir {
                Rotation::Clockwise => {
                    block.x = self.template.center_point.x + y;
                    block.y = self.template.center_point.y - x;
                }
                Rotation::CounterClockwise => {
                    block.x = self.template.center_point.x - y;
                    block.y = self.template.center_point.y + x;
                }
            }
        }
    }
}

pub trait Translate {
    fn translate(&mut self, dir: IVec2);
}

impl Translate for Piece {
    fn translate(&mut self, dir: IVec2) {
        self.position += dir;
    }
}

pub fn rotate_piece(
    transforms: &mut Query<&mut Transform, With<Cube>>,
    piece: &mut Piece,
    dir: Rotation,
    board: &Board,
) {
    let mut new_piece = piece.clone();

    new_piece.rotate(dir);

    let kick_offsets = match new_piece.facing {
        Facing::Up => match dir {
            Rotation::Clockwise => &new_piece.template.kick_offsets.up.clockwise,
            Rotation::CounterClockwise => &new_piece.template.kick_offsets.up.counter_clockwise,
        },
        Facing::Right => match dir {
            Rotation::Clockwise => &new_piece.template.kick_offsets.right.clockwise,
            Rotation::CounterClockwise => &new_piece.template.kick_offsets.right.counter_clockwise,
        },
        Facing::Left => match dir {
            Rotation::Clockwise => &new_piece.template.kick_offsets.left.clockwise,
            Rotation::CounterClockwise => &new_piece.template.kick_offsets.left.counter_clockwise,
        },
        Facing::Down => match dir {
            Rotation::Clockwise => &new_piece.template.kick_offsets.down.clockwise,
            Rotation::CounterClockwise => &new_piece.template.kick_offsets.down.counter_clockwise,
        },
    };

    for &kick_offset in kick_offsets {
        let mut kicked_piece = new_piece.clone();
        kicked_piece.translate(kick_offset);

        if piece_fits(board, &kicked_piece) {
            *piece = kicked_piece;
            break;
        }
    }
    
    update_piece_mesh(piece, transforms);
}

pub fn try_translate_piece(
    transforms: &mut Query<&mut Transform, With<Cube>>,
    piece: &mut Piece,
    dir: IVec2,
    board: &Board,
) -> bool {
    let mut new_piece = piece.clone();
    new_piece.translate(2 * dir);

    if piece_fits(board, &new_piece) {
        *piece = new_piece;
    } else {
        return false
    }
    
    update_piece_mesh(piece, transforms);
    true
}


pub fn can_translate_piece(
    piece: &Piece,
    dir: IVec2,
    board: &Board,
) -> bool {
    for block in get_piece_block_positions(piece) {
        if !block_fits(board, &(block + dir)) {
            return false;
        }
    }
    
    true
}

pub fn move_piece_to(
    transforms: &mut Query<&mut Transform, With<Cube>>,
    piece: &mut Piece,
    pos: IVec2,
    board: &Board,
) {
    let mut new_piece = piece.clone();
    new_piece.move_to(2 * pos);

    if piece_fits(board, &new_piece) {
        *piece = new_piece;
    }
    
    update_piece_mesh(piece, transforms);
}

pub fn update_piece_mesh(
    piece: &mut Piece,
    transforms: &mut Query<&mut Transform, With<Cube>>,
) {
    let blocks = get_piece_block_positions(&piece);
    // println!("piece: {} {}", piece.position.x, piece.position.y);
    for (i, cube_entity) in piece.cubes.iter().enumerate() {
        // println!("\tblock: {}, {}", blocks[i].x, blocks[i].y);
        if let Ok(mut transform) = transforms.get_mut(*cube_entity) {
            transform.translation.x = blocks[i].x as f32;
            transform.translation.y = blocks[i].y as f32;
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
struct RotationKicks {
    clockwise: Vec<IVec2>,
    counter_clockwise: Vec<IVec2>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
struct KickOffsets {
    up: RotationKicks,
    right: RotationKicks,
    down: RotationKicks,
    left: RotationKicks,
}

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        let piece_templates: Vec<PieceTemplate> =
            serde_json::from_reader(
            std::fs::File::open(constants::PIECE_FILE)
                    .expect("unable to open PIECE_FILE")
            ).expect("unable to parse PIECE_FILE");

        app.insert_resource(PieceTemplates(piece_templates));
    }
}

pub fn spawn_piece(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &Res<AssetServer>,
    template: PieceTemplate,
) -> Entity {
    let blocks = template.blocks.clone();

    let piece_entity = commands.spawn_empty().id();

    let mut cube_entities = Vec::new();

    for block in &blocks {
        let cube_entity = commands
            .spawn((
                Cube,
                Mesh3d(meshes.add(Mesh::from(Cuboid::from_length(1.0)))),
                MeshMaterial3d(materials.add(
                    StandardMaterial::from_color(Color::srgb(rand::random(), rand::random(), rand::random())),
                )),
                Transform::from_xyz(
                    (block.x >> 1) as f32,
                    (block.y >> 1) as f32,
                    0.0,
                ),
                GlobalTransform::default(),
            ))
            .id();

        commands.entity(piece_entity).add_child(cube_entity);

        cube_entities.push(cube_entity);
    }

    commands.entity(piece_entity).insert((
        Piece {
            template,
            blocks,
            cubes: cube_entities,
            position: IVec2::default(),
            facing: Facing::Up,
        },
        Transform::default(),
        GlobalTransform::default(),
    ));

    piece_entity
}

/// because the pieces' blocks use a doubled coord system this function
/// exists to convert between them
pub fn get_piece_block_positions(piece: &Piece) -> Vec<IVec2> {
    piece.blocks.iter()
        .map(|pos| ivec2((piece.position.x + pos.x) >> 1, (piece.position.y + pos.y) >> 1))
        .collect()
}

/// because the pieces' blocks use a doubled coord system this function
/// exists to convert between them
pub fn get_piece_template_block_positions(piece: &PieceTemplate) -> Vec<IVec2> {
    piece.blocks.iter()
        .map(|pos| ivec2( pos.x >> 1, pos.y >> 1))
        .collect()
}
