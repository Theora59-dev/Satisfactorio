use bevy::prelude::*;


pub fn make_cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    let cube_mesh = meshes.add(Cuboid::mesh(&Cuboid::new(1.0, 1.0, 1.0)));
    let cube_material = materials.add(Color::srgb(1.0, 0.0, 0.0));

    commands.spawn(PbrBundle {
        mesh: cube_mesh,
        material: cube_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

pub fn make_cubes_of_cubes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    let cube_mesh = meshes.add(Cuboid::mesh(&Cuboid::new(1.0, 1.0, 1.0)));
    let cube_material = materials.add(Color::srgb_u8(121, 73, 0));

    for x in -30..30{
        for z in -30..30{

            commands.spawn(PbrBundle {
                mesh: cube_mesh.clone(),
                material: cube_material.clone(),
                transform: Transform::from_xyz(x as f32, 0.0, z as f32),
                ..default()
            });
        }
    }
}