use std::{collections::VecDeque, time::Duration};

use bevy::{asset::RenderAssetUsages, mesh::PrimitiveTopology, prelude::*};

use crate::{piece::*};

#[derive(Component, Debug)]
pub struct Board {
    width: i32,
    height: i32,
    state: Vec<bool>,
    state_entities: Vec<Option<Entity>>,
    piece_queue: VecDeque<Entity>,
    piece_queue_len: usize,
    piece_queue_position: Vec3,
    piece_spawn_point: IVec2,
    active_piece: Option<Entity>,
    active_piece_place_timer: Timer,
    active_piece_down_movement_timer: Timer,
}

impl Board {
    pub fn new(width: i32, height: i32, piece_queue_len: usize, piece_queue_position: Vec3, piece_spawn_point: IVec2, active_piece_place_duration: Duration, active_piece_down_movement_duration: Duration) -> Self {
        Self {
            width,
            height,
            state: vec![false; (width * height) as usize],
            state_entities: vec![Option::None; (width * height) as usize],
            piece_queue: VecDeque::new(),
            piece_queue_len,
            piece_queue_position,
            piece_spawn_point,
            active_piece: Option::None,
            active_piece_place_timer: Timer::new(active_piece_place_duration, TimerMode::Once),
            active_piece_down_movement_timer: Timer::new(active_piece_down_movement_duration, TimerMode::Repeating),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self { 
            width: 10, 
            height: 30,
            state: vec![false; 10 * 30], 
            state_entities: vec![Option::None; 10 * 30],
            piece_queue: VecDeque::new(), 
            piece_queue_len: 5, 
            piece_queue_position: Vec3 { x: 14.0, y: 30.0, z: 0.0 },
            piece_spawn_point: IVec2 { x: 5, y: 23 },
            active_piece: Option::None,
            active_piece_place_timer: Timer::from_seconds(1.5, TimerMode::Once),
            active_piece_down_movement_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "piece_queue_len: {} piece_queue_position: {} piece_queue: {:?}", self.piece_queue_len, self.piece_queue_position, self.piece_queue)?;
        // writeln!(f, "active_piece: {:?} active_piece_place_timer: {:?} active_piece_down_movement_timer: {:?}", self.active_piece, self.active_piece_place_timer, self.active_piece_down_movement_timer)?;
        // writeln!(f, "width: {} height: {} piece_spawn_point: {}", self.width, self.height, self.piece_spawn_point)?;
        writeln!(f, "{}", "-".repeat((self.width + 4) as usize))?;
        for y in (0..(self.height>>1)).rev() {
            write!(f, "| ")?;
            for x in 0..self.width {
                write!(f, "{}", match self.state[(y * self.width + x) as usize] {
                    true => "█",
                    false => "_",
                })?;
            }
            writeln!(f, " |")?;
        }
        writeln!(f, "{}", "-".repeat((self.width + 4) as usize))?;

        // writeln!(f, "{}", "-".repeat((self.width + 4) as usize))?;
        // for y in (0..(self.height>>1)).rev() {
        //     write!(f, "| ")?;
        //     for x in 0..self.width {
        //         write!(f, "{}", match self.state_entities[(y * self.width + x) as usize] {
        //             Some(_entity) => "█",
        //             None => "_",
        //         })?;
        //     }
        //     writeln!(f, " |")?;
        // }
        // writeln!(f, "{}", "-".repeat((self.width + 4) as usize))?;

        Ok(())
    }
}

pub fn setup(
    piece_templates: Res<PieceTemplates>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let board = Board::default();

    setup_board(board, &piece_templates, &mut commands, &asset_server, &mut meshes, &mut materials);
}

pub fn setup_board(
    mut board: Board,
    piece_templates: &Res<PieceTemplates>,
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>
) {
    setup_board_queue(&mut board, piece_templates, &mut commands, &mut meshes, asset_server, &mut materials);

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
    // temp debug axii 
    {
        let pos = Vec3::new(-2.0, 0.0, -1.0);
        commands.spawn((
            Mesh3d(meshes.add(Segment3d::new(Vec3::ZERO, Vec3::X))),
            Transform::from_translation(pos),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color : Color::srgb(1.0, 0.0, 0.0),
                unlit : true,
                ..Default::default()
            })),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Segment3d::new(Vec3::ZERO, Vec3::Y))),
            Transform::from_translation(pos),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color : Color::srgb(0.0, 1.0, 0.0),
                unlit : true,
                ..Default::default()
            })),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Segment3d::new(Vec3::ZERO, Vec3::Z))),
            Transform::from_translation(pos),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color : Color::srgb(0.0, 0.0, 1.0),
                unlit : true,
                ..Default::default()
            })),
        ));
    }
}

pub fn setup_board_queue(
    board: &mut Board,
    piece_templates: &Res<PieceTemplates>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    // add pieces to piece queue
    for piece in piece_templates.0.iter().chain(piece_templates.0.iter()) {
        let piece_entity = spawn_piece(commands, meshes, materials, &asset_server, piece.clone());
        board.piece_queue.push_back(piece_entity);
    }
    update_board_queue(&board, commands);
}

pub fn new_piece_system(
    mut boards: Query<&mut Board>,
    mut pieces: Query<&mut Piece>,
    mut transforms: Query<&mut Transform, With<Cube>>,
    piece_templates: Res<PieceTemplates>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for mut board in boards.iter_mut() {
        if board.active_piece.is_some() {
            continue;
        }

        let piece_entity = board.piece_queue.pop_front().unwrap();
    
        let mut piece = pieces.get_mut(piece_entity).unwrap();
    
        let piece_placed = place_piece_in_board(
            &mut commands,
            &mut transforms,
            &mut board,
            &mut piece,
            piece_entity,
        );
    
        if !piece_placed {
            eprintln!("unable to place piecce");
        }
    
        // add more pieces if queue is running out
        if board.piece_queue.len() < board.piece_queue_len * 2  {
            for piece in piece_templates.0.iter() {
                let piece_entity = spawn_piece(&mut commands, &mut meshes, &mut materials, &asset_server, piece.clone());
                board.piece_queue.push_back(piece_entity);
            }
        }

        update_board_queue(&board, &mut commands);
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

pub fn piece_fits(
    board: &Board,
    piece: &Piece,
) -> bool {
    for block in get_piece_block_positions(piece) {
        if cell_occupied(board, block.x, block.y) {
            return false;
        }
    }

    true
}

pub fn block_fits(
    board: &Board,
    block: &IVec2,
) -> bool {
    if cell_occupied(board, block.x, block.y) {
        return false;
    }

    true
}

pub fn cell_occupied(board: &Board, x: i32, y: i32) -> bool {
    if x < 0 || x >= board.width || y < 0 || y >= board.height {
        return true;
    }

    board.state[(x + y * board.width) as usize]
}

pub fn place_blocks(
    board: &mut Board, 
    piece: &Piece,
    piece_entity: Entity,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    for (i, block) in get_piece_block_positions(piece).iter().enumerate() {
        let index = ((block.y * board.width) + block.x) as usize; 
        let block_entity = piece.cubes[i];

        let new_board_entity = create_board_block_entity(meshes, materials, commands, block_entity, block);

        board.state[index] = true;
        board.state_entities[index] = Some(new_board_entity);
    }

    board.active_piece = Option::None;
    commands.entity(piece_entity).despawn();
}

#[derive(Component)]
pub struct BoardBlock;

pub fn create_board_block_entity(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    commands: &mut Commands,
    previous_block_entity: Entity,
    position: &IVec2,
) -> Entity {
    commands.spawn((
        BoardBlock,
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            ..Default::default()
        })),
        Transform::from_xyz(position.x as f32 + 0.5, position.y as f32 + 0.5, -0.5),
    )).id()
}

pub fn place_piece_in_board(
    commands: &mut Commands,
    transforms: &mut Query<&mut Transform, With<Cube>>,
    board: &mut Board,
    piece: &mut Piece,
    piece_entity: Entity,
) -> bool {
    piece.move_to(board.piece_spawn_point);

    for pos in get_piece_block_positions(piece) {
        if cell_occupied(board, pos.x, pos.y) {
            return false;
        }
    }
    // move piece to correct spot
    // TODO: set to board transform
    commands.entity(piece_entity).insert(Transform::from_xyz(0.5, 0.5, -0.5));

    move_piece_to(transforms, piece, ivec2(board.piece_spawn_point.x, board.piece_spawn_point.y), board);

    make_active_piece(board, piece_entity);

    true
}


fn make_active_piece(board: &mut Board, piece: Entity) {
    // set piece to be board's active piece (so it can be searched for later)
    board.active_piece = Some(piece);
}

pub fn slam_piece_down(transforms: &mut Query<&mut Transform, With<Cube>>, piece: &mut Piece, board: &mut Board) {
    while try_translate_piece(transforms, piece, -IVec2::Y, board) {}

    board.active_piece_place_timer.finish();
}

pub fn move_and_rotate_piece_system(
    mut boards: Query<&mut Board>,
    mut pieces: Query<&mut Piece>,
    mut transforms: Query<&mut Transform, With<Cube>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for mut board in boards.iter_mut() {
        if board.active_piece.is_none() { 
            continue; 
        }

        let piece_entity = board.active_piece.unwrap();

        let mut piece = pieces.get_mut(piece_entity).unwrap();

        // clockwise rotation
        if input.just_pressed(KeyCode::KeyI) {
            _ = rotate_piece(
                &mut transforms,
                &mut piece,
                Rotation::Clockwise,
                &board,
            );
        }
        // counter clockwise rotation
        if input.just_pressed(KeyCode::KeyK) {
            _ = rotate_piece(
                &mut transforms,
                &mut piece,
                Rotation::CounterClockwise,
                &board,
            );
        }

        // translate piece left
        if input.just_pressed(KeyCode::KeyJ) {
            _ = try_translate_piece(
                &mut transforms,
                &mut piece,
                -IVec2::X,
                &board,
            )
        }
        // translate piece right
        if input.just_pressed(KeyCode::KeyL) {
            _ = try_translate_piece(
                &mut transforms,
                &mut piece,
                IVec2::X,
                &board,
            )
        }

        if input.just_pressed(KeyCode::Space) {
            slam_piece_down(
                &mut transforms, 
                &mut piece,
                &mut board,
            );
        }
    }
}

pub fn move_piece_down_system(
    mut boards: Query<&mut Board>,
    mut pieces: Query<&mut Piece>,
    mut transforms: Query<&mut Transform, With<Cube>>,
    time: Res<Time>,
) {
    for mut board in boards.iter_mut() {
        if board.active_piece.is_none() { 
            continue; 
        }
        
        board.active_piece_down_movement_timer.tick(time.delta());
        
        if board.active_piece_down_movement_timer.just_finished() {
            let piece_entity = board.active_piece.unwrap();
            
            let mut piece = pieces.get_mut(piece_entity).unwrap();
            
            _ = try_translate_piece(&mut transforms, &mut piece, -IVec2::Y, &board)
        }
    }
}

pub fn place_piece_system(
    mut boards: Query<&mut Board>,
    mut pieces: Query<&mut Piece>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for mut board in boards.iter_mut() {
        if board.active_piece.is_none() { 
            continue; 
        }

        let piece_entity = board.active_piece.unwrap();
        
        let piece = pieces.get_mut(piece_entity).unwrap();

        if can_translate_piece(&piece, -IVec2::Y, &board) {
            board.active_piece_place_timer.reset();
            continue;
        }
        
        board.active_piece_place_timer.tick(time.delta());

        if board.active_piece_place_timer.is_finished() {            
            place_blocks(
                &mut board, 
                &piece, 
                piece_entity, 
                &mut commands, 
                &mut meshes, 
                &mut materials
            );
        }
    }
}

fn clear_row(board: &mut Board, y: i32, commands: &mut Commands) {
    for x in 0..board.width {
        // clear the line
        let block_index = (y * board.width + x) as usize;
        board.state[block_index] = false;
        if let Some(block_entity) = board.state_entities[block_index] {
            commands.entity(block_entity).despawn();
        }
        board.state_entities[block_index] = Option::None;
    }

    // move above blocks down 1 
    for y in y+1..board.height {
        for x in 0..board.width {
            let block_index = (y * board.width + x) as usize;
            let block_below_index = ((y - 1) * board.width + x) as usize;

            board.state[block_below_index] = board.state[block_index];
            board.state_entities[block_below_index] = board.state_entities[block_index];
        }
    }
}

pub fn clear_lines_system(
    mut boards: Query<&mut Board>,
    mut transforms: Query<&mut Transform, With<BoardBlock>>,
    mut commands: Commands,
) {
    for mut board in &mut boards {
        'row: for y in (0..board.height).rev() {
            for x in 0..board.width {
                let index = (x + y * board.width) as usize;
                if !board.state[index] {
                    continue 'row
                }
            }
            
            // board row is filled
            clear_row(&mut board, y, &mut commands);

            update_board_entities(
                &board,
                &mut transforms,
            );
        }
    }
}

fn update_board_entities(
    board: &Board,
    transforms: &mut Query<&mut Transform, With<BoardBlock>>,
) {
    for y in 0..board.height {
        for x in 0..board.width {
            let index = (x + y * board.width) as usize;
            if let Some(block_entity) = board.state_entities[index] {
                if let Ok(mut transform) = transforms.get_mut(block_entity) {
                    print!("{} {}", x, y);
                    transform.translation = Vec3 { x: x as f32 + 0.5, y: y as f32 + 0.5, z: -0.5 };
                }
            }
        }
    }
}
