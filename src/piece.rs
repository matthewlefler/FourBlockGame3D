use bevy::{prelude::*};
use serde::{Deserialize, Serialize};

const PIECE_FILE: &str = "./src/data/pieces.json";

#[derive(Resource)]
pub struct PieceTemplates(pub Vec<PieceTemplate>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PieceTemplate {
    pub name: String,
    blocks: Vec<IVec2>,
    center_point: IVec2,
    kick_offsets: KickOffsets,
}

#[derive(Component, Clone, Debug)]
pub struct Piece {
    template: PieceTemplate,
    pub blocks: Vec<IVec2>,
    cubes: Vec<Entity>,
    facing: Facing,
}

#[derive(Component)]
pub struct Cube;

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Clockwise,
    CounterClockwise
}

#[derive(Clone, Debug)]
enum Facing {
    Up,
    Right,
    Down,
    Left,
}

trait Rotate {
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

fn rotate_piece_system(
    mut pieces: Query<&mut Piece, With<Active>>,
    mut transforms: Query<&mut Transform, With<Cube>>,
) {
    for mut piece in &mut pieces {
        piece.template.rotate(Rotation::Clockwise);

        for (i, cube_entity) in piece.cubes.iter().enumerate() {
            if let Ok(mut transform) = transforms.get_mut(*cube_entity) {
                transform.translation.x =
                    piece.template.blocks[i].x as f32;

                transform.translation.y =
                    piece.template.blocks[i].y as f32;
            }
        }
    }
}

#[derive(Debug, Component)]
pub struct Active;

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
            serde_json::from_reader(std::fs::File::open(PIECE_FILE).expect("unable to open PIECE_FILE")).expect("unable to parse PIECE_FILE");
        app.insert_resource(PieceTemplates(piece_templates));
    }
}

pub fn spawn_piece(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
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
                    StandardMaterial::from_color(Color::srgb(1.0, 0.5, 0.0)),
                )),
                Transform::from_xyz(
                    block.x as f32 / 2.0,
                    block.y as f32 / 2.0,
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
            facing: Facing::Up,
        },
        Transform::default(),
        GlobalTransform::default(),
    ));

    piece_entity
}

pub fn make_active(piece: Entity, commands: &mut Commands) {
    commands.entity(piece).insert_if_new(Active );
}