use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Debug, Resource)]
pub struct CameraRotRelative(pub f32);



pub fn move_camera(mut mouse_input: EventReader<MouseMotion>, mut query: Query<&mut Transform, With<Camera>>, mut camera_rot_relative: ResMut<CameraRotRelative>) {
    let sensitivity = 0.00048828125;

    for mut transform in query.iter_mut() {

        // Orientation Caméra
        for ev in mouse_input.read() {
            // Axe Vertical
            let mut rot_x: f32 = -ev.delta.y * sensitivity;
            if camera_rot_relative.0 > 2.2{
                println!("Est trop loin");
                rot_x = -0.001;
            } 
            else if camera_rot_relative.0 < -0.7{
                rot_x = 0.001;

            }
                
            camera_rot_relative.0 += rot_x;
            println!("Coordonnées relatives de la caméra: {}", camera_rot_relative.0);
            transform.rotate_local_x(rot_x);
            

            // Axe Horizontal
            transform.rotate_y(-ev.delta.x * sensitivity);
            
            // Debug pour la rotation
            // Il faut qu'on arrive à clamp l'axe vertical pour pas se faire un torticoli car là ça devient chaud miskine le joueur il meurt h24 😭😭😭
            // println!("X: {} Y: {} Z: {} W: {}", transform.rotation.x, transform.rotation.y, transform.rotation.z, transform.rotation.w/*, transform.local_y().z*/);
        }

    }
}

