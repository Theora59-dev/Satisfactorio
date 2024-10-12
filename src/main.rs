mod camera;
mod movement;
mod debug_controls;
mod world_generator;

use movement::*;
use debug_controls::*;
use world_generator::*;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, WindowMode};
use camera::*;



fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(LoadedChunks::default())
    .insert_resource(ClearColor(Color::srgb_u8(37, 179, 226)))
    .add_systems(Startup, (
        start,
        setup_light_world,
    ),
                
    )
    .add_systems(Update,(
        move_camera,
        display_chunk_mesh,
        move_player,
        check_debug_controls,
    ))
    .run();

}


fn start(mut commands: Commands, mut windows: Query<&mut Window>) {

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 25.0, -20.0).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
        ..default()
    });
    

    if let Ok(mut window) = windows.get_single_mut() {
        window.mode = WindowMode::Fullscreen;
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Confined;
    }  



    
}
