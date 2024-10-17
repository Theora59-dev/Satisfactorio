use bevy::prelude::*;

pub fn move_player(keyboard_input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in query.iter_mut() {

        let mut direction = Vec3::ZERO;
        let mut speed: f32 = 15.0;

        if keyboard_input.pressed(KeyCode::ControlLeft){
            speed = 250.0;
        }

        // direction avant-arrière:
        // (les customs transforms servent éviter de controller l'altitude avec les touches Z et S 
        // comme dans minecraft en créatif plutôt qu'en spectateur)
        if keyboard_input.pressed(KeyCode::KeyW) {
            let mut custom_transform = *transform.forward();
            custom_transform.y = 0.0;
            direction += custom_transform ;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let mut custom_transform = *transform.forward();
            custom_transform.y = 0.0;
            direction -= custom_transform ;
        }
        // Direction gauche-droite
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= *transform.right() ;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += *transform.right() ;  
        }

        // Direction haut-bas
        if keyboard_input.pressed(KeyCode::ShiftLeft){
            direction.y -= 1.5;
        }

        if keyboard_input.pressed(KeyCode::Space){
            direction.y += 1.5;
        }

        
        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * speed * time.delta_seconds();
        }

    }
}