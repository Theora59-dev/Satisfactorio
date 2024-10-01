mod camera;

use bevy::prelude::*;
use camera::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,
        start
    )
    .add_systems(Update,
        move_camera
    )
    .run();

}


fn start(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Caméra
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
        ..default()
    });

    // Cube
    let mesh = meshes.add(Cuboid::mesh(&Cuboid::new(5.0, 10.0, 5.0)));
    let material = materials.add(Color::srgb(1.0, 0.0, 0.0));

    commands.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    });
}
