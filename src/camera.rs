use bevy::{input::mouse::MouseMotion, prelude::*, window::WindowTheme};




pub fn move_camera(keyboard_input: Res<ButtonInput<KeyCode>>, mut mouse_input: EventReader<MouseMotion>, _time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    let sensitivity = 0.00048828125;
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;  

        for ev in mouse_input.read() {
            transform.rotate_x(ev.delta.y * sensitivity);
            transform.rotate_local_y(-ev.delta.x * sensitivity);
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;  
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * 0.125;
    }
}

