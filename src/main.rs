use bevy::prelude::*;
mod sprite_movement;

#[derive(Component)]
struct Player;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, sprite_movement::sprite_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut materials: ResMut<Assets<ColorMaterial>>,

) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Player)
        .insert(SpriteBundle {
            texture: asset_server.load("../assets/player.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..SpriteBundle::default()
        })
        .insert(sprite_movement::Movement {
            up: false,
            down: false,
            left: false,
            right: false,
        });
}

