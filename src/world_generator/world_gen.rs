use std::collections::{HashSet, VecDeque};
use bevy::prelude::*;
use super::chunks::*;

pub fn _make_cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    let cube_mesh = meshes.add(Cuboid::mesh(&Cuboid::new(1.0, 1.0, 1.0)));
    let cube_material = materials.add(Color::srgb(1.0, 0.0, 0.0));

    commands.spawn(PbrBundle {
        mesh: cube_mesh,
        material: cube_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}


#[derive(Default, Resource)]
pub struct ChunkManager {
    // Éléments à charger (liste avc coos de chunks)
    chunks_to_load: HashSet<(i32, i32)>,
    // Éléments chargés (liste avc entités et coos de chunks)
    chunks_loaded: VecDeque<(Entity, (i32, i32))>,
}


pub fn display_chunk_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Transform), With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    const CHUNK_SIZE: f32 = 32.0;
    const RENDER_DISTANCE: i32 = 16;


    let (_camera_entity, camera_transform) = query.single();
    let player_position = camera_transform.translation;
    let player_chunk_x = (player_position.x / CHUNK_SIZE).floor() as i32;
    let player_chunk_z = (player_position.z / CHUNK_SIZE).floor() as i32;

    // Charger les nouveaux chunks nécessaires
    for dx in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for dz in -RENDER_DISTANCE..=RENDER_DISTANCE {
            let chunk_x = player_chunk_x + dx;
            let chunk_z = player_chunk_z + dz;

            if !chunk_manager.chunks_to_load.contains(&(chunk_x, chunk_z)) {
                let entities = Chunk::new(CHUNK_SIZE as usize, chunk_x, chunk_z).create_chunk_mesh(&mut commands, &mut meshes, &mut materials);
                chunk_manager.chunks_to_load.insert((chunk_x, chunk_z));
                chunk_manager.chunks_loaded.extend(entities.into_iter().map(|entity| (entity, (chunk_x, chunk_z))));
            }
        }
    }

    
}


pub fn unload_chunks(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
){
    const UNLOAD_DISTANCE: i32 = 16;
    const CHUNK_SIZE: f32 = 32.0;

    let (_camera_entity, camera_transform) = query.single();
    let player_position = camera_transform.translation;
    let player_chunk_x = (player_position.x / CHUNK_SIZE).floor() as i32;
    let player_chunk_z = (player_position.z / CHUNK_SIZE).floor() as i32;

    let mut chunks_to_remove = Vec::new();

    chunk_manager.chunks_loaded.retain(|(entity, (chunk_x, chunk_z))| {
        let distance_to_player = ((player_chunk_x - *chunk_x).abs() + (player_chunk_z - *chunk_z).abs()) / 2;
        if distance_to_player > UNLOAD_DISTANCE {
            if let Some(mut entity_commands) = commands.get_entity(*entity) {
                entity_commands.despawn();
            }
            chunks_to_remove.push((*chunk_x, *chunk_z));
            false
        } else {
            true
        }
    });

    for chunk_coords in chunks_to_remove {
        chunk_manager.chunks_to_load.remove(&chunk_coords);
    }
}


