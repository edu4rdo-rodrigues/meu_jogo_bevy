use bevy::prelude::CursorMoved;
use bevy::prelude::*;
use bevy::ecs::schedule::SystemSet;
use bevy::ecs::prelude::{Res, ResMut, Resource};

mod sprite_movement;


#[derive(Component)]
struct Player;

#[derive(Component)]
struct Projectile;


#[derive(Debug)]
pub struct MousePosition {
    pub position: Vec2,
}


// Defina a estrutura para armazenar os valores da tela
#[derive(Debug, Default)]
pub struct ScreenSize {
    pub width: f32,
    pub height: f32,
}

// Implemente o trait Resource para ScreenSize
impl Resource for ScreenSize {}



// Variável global para armazenar os valores das posições do mouse
static mut MOUSE_POSITION: Option<Vec2> = None;

static mut SCREEN_SIZE: Option<ScreenSize> = None;

pub fn mouse_position_system(
    mut cursor_mover_events: EventReader<CursorMoved>,
) {
    for event in cursor_mover_events.read() {
        let mouse_position = Vec2::new(event.position.x, event.position.y);
        //println!("Mouse Position: Vec2({}, {})", event.position.x, event.position.y);

        // Armazenar os valores na variável global
        unsafe {
            MOUSE_POSITION = Some(mouse_position);
        }
    }
}


// Função para imprimir o tamanho da tela
fn print_screen_size(
    query: Query<&Window>, 
    mut screen_size: ResMut<ScreenSize>
) {
    if let Some(window) = query.iter().next() {
        let width = window.width();
        let height = window.height();
        let new_screen_size = ScreenSize { width, height };

        // Atualiza o recurso ScreenSize
        *screen_size = new_screen_size;

        println!("Screen Size: {} x {}", width, height);
    }
}



fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    screen_size: Res<ScreenSize>,
) {

    if mouse_button_input.just_pressed(MouseButton::Left) {
        println!("Left mouse button clicked!");
        // Recuperar a variável global MOUSE_POSITION
        unsafe {
            if let Some(mouse_position) = MOUSE_POSITION {
                println!("Mouse Position: Vec2({}, {})", mouse_position.x, mouse_position.y);
                    // Obtém os valores de largura e altura da tela armazenados globalmente
                let width = screen_size.width / 2.0;
                let height = screen_size.height / 2.0;
                let widthdff = mouse_position.x - width;
                let heightdff = mouse_position.y - height;

                println!("Mouse clicked at width: {}", widthdff);

                commands.spawn(
                    SpriteBundle {
                    texture: asset_server.load("../assets/player.png"),
                    transform: Transform::from_translation(Vec3::new(widthdff, -heightdff, 0.)),
                    ..SpriteBundle::default()
                }).insert(Projectile);

            }
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        println!("Right mouse button clicked!");
    }
    
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ScreenSize{width: 1200.0, height: 600.0})
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, sprite_movement::sprite_movement)
        .add_systems(FixedUpdate, mouse_position_system)
        .add_systems(FixedUpdate, mouse_click_system)
        .add_systems(FixedUpdate, print_screen_size)
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

    // Inicializar a variável global MOUSE_POSITION
    unsafe {
        MOUSE_POSITION = Some(Vec2::new(0.0, 0.0));
    }
    unsafe {
        SCREEN_SIZE = Some(ScreenSize { width: 0.0, height: 0.0 });
    }
        
}

