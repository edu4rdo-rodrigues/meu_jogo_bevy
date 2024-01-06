use bevy::prelude::*;

#[derive(Component)]
struct Player;



#[derive(Component)]
struct Movement {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, sprite_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut materials: ResMut<Assets<ColorMaterial>>,

) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("../assets/player.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..SpriteBundle::default()
        })
        .insert(Player)
        .insert(Movement {
            up: false,
            down: false,
            left: false,
            right: false,
        });
}

fn sprite_movement(
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