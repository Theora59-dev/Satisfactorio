use bevy::prelude::*;
use bevy::math::vec2;


pub fn _world_meshes_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    // Cube
    let mesh1 = meshes.add(Cuboid::mesh(&Cuboid::new(5.0, 10.0, 5.0)));
    let material1 = materials.add(Color::srgb(1.0, 0.0, 0.0));

    commands.spawn(PbrBundle {
        mesh: mesh1,
        material: material1,
        transform: Transform::from_xyz(0.0, 5.0, 10.0),
        ..default()
    });

    

    // Cube
    let mesh2 = meshes.add(Cuboid::mesh(&Cuboid::new(5.0, 10.0, 5.0)));
    let material2 = materials.add(Color::srgb(0.0, 1.0, 0.0));

    commands.spawn(PbrBundle {
        mesh: mesh2,
        material: material2,
        transform: Transform::from_xyz(10.0, 5.0, 0.0),
        ..default()
    });

    // Cube
    let mesh3 = meshes.add(Cuboid::mesh(&Cuboid::new(5.0, 10.0, 5.0)));
    let material3 = materials.add(Color::srgb(1.0, 1.0, 0.0));

    commands.spawn(PbrBundle {
        mesh: mesh3,
        material: material3,
        transform: Transform::from_xyz(-10.0, 5.0, 0.0),
        ..default()
    });

    // Cube
    let mesh4 = meshes.add(Cuboid::mesh(&Cuboid::new(5.0, 10.0, 5.0)));
    let material4 = materials.add(Color::srgb(0.0, 0.0, 1.0));

    commands.spawn(PbrBundle {
        mesh: mesh4,
        material: material4,
        transform: Transform::from_xyz(0.0, 5.0, -10.0),
        ..default()
    });


    let plane = meshes.add(Plane3d::new(Vec3 { x: 0.0, y: 1.0, z: 0.0 }, vec2(500.0, 500.0)));
    let material4 = materials.add(Color::srgb(0.0, 0.5, 0.0));
    commands.spawn(PbrBundle {
        mesh: plane,
        material: material4,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}


pub fn setup_light_world(mut commands: Commands){
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(1.0, 1.0, 1.0),
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(20.0, 15.0, -30.0))
            .looking_at(Vec3::NEG_Z, Vec3::Y),
        ..default()
    });


}
    