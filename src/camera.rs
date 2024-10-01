use bevy::{input::mouse::MouseMotion, prelude::*};






pub fn move_camera(keyboard_input: Res<ButtonInput<KeyCode>>, mut mouse_input: EventReader<MouseMotion>, _time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;  

        for ev in mouse_input.read() {
            let mut rotation = transform.rotation.to_euler(EulerRot::XYZ);

            rotation.1 += ev.delta.x * 0.00048828125; // constante de sensibilité

            rotation.0 += -ev.delta.y * 0.00048828125; // constante de sensibilité
            rotation.0 = rotation.0.clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);

            transform.rotation = Quat::from_euler(EulerRot::YXZ, rotation.1, rotation.0, 0.0);

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