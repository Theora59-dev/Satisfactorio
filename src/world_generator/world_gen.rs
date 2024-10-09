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

pub fn display_chunk_mesh(commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<StandardMaterial>>){
    let mut chunk = Chunk::new(32);
    create_chunk_mesh(&mut chunk, commands, meshes, materials);

    


}



