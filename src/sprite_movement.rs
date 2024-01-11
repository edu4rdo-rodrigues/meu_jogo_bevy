// sprite_movement.rs

use bevy::prelude::*;


#[derive(Debug, Default, Component)]  // Use #[derive(Component)] para derivar automaticamente
pub struct Movement {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}



pub fn sprite_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Movement, &mut Transform)>,
) {
    for (mut movement, mut transform) in &mut query.iter_mut() {
        let mut new_translation = transform.translation;

        if keyboard_input.pressed(KeyCode::W) {
            movement.up = true;
        } else {
            movement.up = false;
        }

        if keyboard_input.pressed(KeyCode::S) {
            movement.down = true;
        } else {
            movement.down = false;
        }

        if keyboard_input.pressed(KeyCode::A) {
            movement.left = true;
        } else {
            movement.left = false;
        }

        if keyboard_input.pressed(KeyCode::D) {
            movement.right = true;
        } else {
            movement.right = false;
        }

        if movement.up {
            new_translation.y += 5.0;
        }

        if movement.down {
            new_translation.y -= 5.0;
        }

        if movement.left {
            new_translation.x -= 5.0;
        }

        if movement.right {
            new_translation.x += 5.0;
        }

        transform.translation = new_translation;
    }
}
