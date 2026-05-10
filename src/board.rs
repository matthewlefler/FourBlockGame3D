use std::{collections::VecDeque};

use bevy::{asset::RenderAssetUsages, mesh::PrimitiveTopology, prelude::*};

use crate::{piece::*};

#[derive(Component, Debug)]
pub struct Board {
    width: i32,
    height: i32,
    state: Vec<bool>,
    piece_queue: VecDeque<Entity>,
    piece_queue_len: usize,
    piece_queue_position: Vec3,
    piece_spawn_point: IVec2,
    active_piece: Option<Entity>,
}

impl Board {
    pub fn new(width: i32, height: i32, piece_queue_len: usize, piece_queue_position: Vec3, piece_spawn_point: IVec2) -> Self {
        Self {
            width,
            height,
            state: vec![false; (width * height) as usize],
            piece_queue: VecDeque::new(),
            piece_queue_len,
            piece_queue_position,
            piece_spawn_point,
            active_piece: Option::None,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self { 
            width: 10, 
            height: 30,
            state: vec![false; (10 * 30) as usize], 
            piece_queue: VecDeque::new(), 
            piece_queue_len: 5, 
            piece_queue_position: Vec3 { x: 14.0, y: 30.0, z: 0.0 },
            piece_spawn_point: IVec2 { x: 5, y: 23 },
            active_piece: Option::None,
        }
    }
}

pub fn setup(
    piece_templates: Res<PieceTemplates>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>
) {
    let board = Board::default();

    setup_board(board, piece_templates, commands, meshes, materials);
}

pub fn setup_board(
    mut board: Board,
    piece_templates: Res<PieceTemplates>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    setup_board_queue(&mut board, piece_templates, &mut commands, &mut meshes, &mut materials);

    // mesh
    let bottom_left = Vec3::ZERO;
    let bottom_right = bottom_left + Vec3::X * board.width as f32;
    let top_left = bottom_left + Vec3::Y * board.height as f32;
    let top_right = bottom_right + Vec3::Y * board.height as f32;

    let mat = StandardMaterial {
        base_color : Color::WHITE,
        unlit : true,
        ..Default::default()
    };

    let z_vec_offset = -Vec3::Z;

    let mesh = Mesh::new(
    PrimitiveTopology::LineList, // Or LineStrip
    RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Define pairs of points (start, end)
        vec![
            bottom_left, bottom_right,
            bottom_left, top_left,
            bottom_right, top_right,

            bottom_left + z_vec_offset, bottom_right + z_vec_offset,
            bottom_left + z_vec_offset, top_left + z_vec_offset,
            bottom_right + z_vec_offset, top_right + z_vec_offset,
        ],
    );

    commands.spawn((
        board,
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(mat)),
    ));
}

pub fn setup_board_queue(
    board: &mut Board,
    piece_templates: Res<PieceTemplates>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    // add pieces to piece queue
    for piece in piece_templates.0.iter().chain(piece_templates.0.iter()) {
        let piece_entity = spawn_piece(commands, meshes, materials, piece.clone());
        board.piece_queue.push_back(piece_entity);
    }
    update_board_queue(&board, commands);
}

pub fn board_new_piece_system(
    mut boards: Query<&mut Board>,
    pieces: Query<&Piece>,
    piece_templates: Res<PieceTemplates>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for mut board in boards.iter_mut() {
        let piece_entity = board.piece_queue.pop_front().unwrap();

        let piece = pieces.get(piece_entity).unwrap();

        let piece_placed = place_piece_in_board(
            &mut commands,
            &mut board,
            piece,
            piece_entity,
        );

        if !piece_placed {
            println!("unable to place piecce");
        }

        // add more pieces if queue is running out
        if board.piece_queue.len() < board.piece_queue_len * 2  {
            for piece in piece_templates.0.iter() {
                let piece_entity = spawn_piece(&mut commands, &mut meshes, &mut materials, piece.clone());
                board.piece_queue.push_back(piece_entity);
            }
            update_board_queue(&board, &mut commands);
        }
    }
}

fn update_board_queue(board: &Board, commands: &mut Commands) {
    for (i, piece) in board.piece_queue.iter().enumerate() {
        let mut entity = commands.entity(*piece);
        let y_offset = Vec3::Y * 3.0 * i as f32;
        entity.insert(Transform::from_translation(board.piece_queue_position + y_offset));
        
        if i > board.piece_queue_len - 1 {
            entity.insert(Visibility::Hidden);
        } else {
            entity.insert(Visibility::Visible);
        }
    }
}

pub fn cell_occupied(board: &Board, x: i32, y: i32) -> bool {
    if x < 0 || x >= board.width || y < 0 || y >= board.height {
        return true;
    }

    board.state[(x + y * board.width) as usize]
}

pub fn place_blocks(board: &mut Board, blocks: &Vec<IVec2>) {
    for block in blocks {
        board.state[((block.x >> 1) + (block.y >> 1) * board.width) as usize] = true;
    }
}

pub fn place_piece_in_board(
    commands: &mut Commands,
    board: &mut Board,
    piece: &Piece,
    piece_entity: Entity,
) -> bool {
    for pos in get_piece_block_positions(piece) {
        let board_pos = pos + board.piece_spawn_point;

        if cell_occupied(board, board_pos.x, board_pos.y) {
            return false;
        }
    }
    // move piece to correct spot
    commands.entity(piece_entity).insert(Transform::from_xyz(board.piece_spawn_point.x as f32, board.piece_spawn_point.y as f32, 0.0));

    make_active_piece(board, piece_entity);

    true
}

fn make_active_piece(board: &mut Board, piece: Entity) {
    // set piece to be board's active piece (so it can be searched for later)
    board.active_piece = Some(piece);
}

pub fn move_and_rotate_piece_system(
    boards: Query<&Board>,
    mut pieces: Query<&mut Piece>,
    mut transforms: Query<&mut Transform, With<Cube>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for board in boards {
        if board.active_piece.is_none() { 
            continue; 
        }

        let piece_entity = board.active_piece.unwrap();

        let mut piece = pieces.get_mut(piece_entity).unwrap();

        // clockwise rotation
        if input.just_pressed(KeyCode::KeyI) {
            rotate_piece(
                &mut transforms,
                &mut piece,
                Rotation::Clockwise,
            );
        }
        // counter clockwise rotation
        if input.just_pressed(KeyCode::KeyK) {
            rotate_piece(
                &mut transforms,
                &mut piece,
                Rotation::CounterClockwise,
            );
        }

        // translate piece left
        if input.just_pressed(KeyCode::KeyJ) {
            translate_piece(
                &mut transforms,
                &mut piece,
                Facing::Left,
            )
        }
        // translate piece right
        if input.just_pressed(KeyCode::KeyL) {
            translate_piece(
                &mut transforms,
                &mut piece,
                Facing::Right,
            )
        }
    }
}