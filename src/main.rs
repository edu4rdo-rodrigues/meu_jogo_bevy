use bevy::math::vec3;
use bevy::prelude::CursorMoved;
use bevy::prelude::*;
use bevy::ecs::prelude::{Res, ResMut, Resource};

mod sprite_movement;


#[derive(Component)]
struct Player;

#[derive(Component)]
struct Projectile;


#[derive(Debug, Default)]
pub struct MousePosition {
    pub position: Vec2,
}

impl Resource for MousePosition {}


// Defina a estrutura para armazenar os valores da tela
#[derive(Debug, Default)]
pub struct ScreenSize {
    pub width: f32,
    pub height: f32,
}

// Implemente o trait Resource para ScreenSize
impl Resource for ScreenSize {}



// Adicione uma nova estrutura de recursos para armazenar a velocidade do projetil
#[derive(Debug, Component)]
pub struct ProjectileSpeed(pub Vec3);


static mut PLAYER_POSITION: Option<Vec3> = Some(Vec3::ZERO);
static mut MOUSE_POSITION: Option<Vec2> = Some(Vec2::ZERO);


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
    }
}

fn monitor_player_position_system(
    screen_size: Res<ScreenSize>,
    query: Query<(&Player, &Transform)>,
) {
    for (_, transform) in query.iter() {
        let width: f32 = screen_size.width / 2.0;
        let height: f32 = screen_size.height / 2.0;

        let player_position_x: f32 = transform.translation.x - width + screen_size.width;
        let player_position_y: f32 = -transform.translation.y + height;
        let player_position_z: f32 = transform.translation.z;


        // Atualiza a variável global PLAYER_POSITION
        unsafe {
            PLAYER_POSITION = Some(vec3(player_position_x, player_position_y, player_position_z));
        };
    }
}


// Adicione um sistema para mover os projéteis com base na velocidade
fn projectile_movement_system(
    time: Res<Time>,
    mut query: Query<(&Projectile, &mut Transform, &ProjectileSpeed)>,
) {
    for (_, mut transform, speed) in query.iter_mut() {
        transform.translation += speed.0 * time.delta_seconds();    }
}

fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    screen_size: Res<ScreenSize>,
    query: Query<(&Player, &Transform)>,

) {

    if mouse_button_input.just_pressed(MouseButton::Left) {
        println!("Left mouse button clicked!");
        // Recuperar a variável global MOUSE_POSITION
        unsafe {
            if let Some(mouse_position) = MOUSE_POSITION {
                //println!("Mouse Position: Vec2({}, {})", mouse_position.x, mouse_position.y);

                // Obtém os valores de largura e altura da tela armazenados globalmente
                let width = screen_size.width / 2.0;
                let height = screen_size.height / 2.0;
                let widthdff = mouse_position.x - width;
                let heightdff = mouse_position.y - height;
                let projectile_position = Vec3::new(widthdff, -heightdff, 0.0);

                //println!("Mouse clicked at width: {}", widthdff);

                    for (_, transform) in query.iter() {
                        // Acessa a posição do jogador a partir do componente Transform
                        let player_position_x: f32 = transform.translation.x;
                        let player_position_y: f32 = transform.translation.y;
                        let player_position_z: f32 = transform.translation.z;

                        let player_positions = Vec3::new(player_position_x, player_position_y, player_position_z);
                        let initial_position = player_positions;
                        let direction = (projectile_position - initial_position).normalize();

                        println!("player_positions: {:?}", player_positions);
                        println!("direction: {:?}", direction);

                        commands.spawn(
                            SpriteBundle {
                            texture: asset_server.load("../assets/player.png"),
                            transform: Transform::from_translation(initial_position),
                            ..SpriteBundle::default()
                        })
                        .insert(Projectile)
                        .insert(ProjectileSpeed(1000.0 * direction));
        
                        if mouse_button_input.just_pressed(MouseButton::Right) {
                            println!("Right mouse button clicked!");
                        }
                
                    }
                
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
        .insert_resource(ScreenSize{width: 1280.0, height: 720.0})
        .insert_resource(MousePosition{ position: Vec2::ZERO })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, sprite_movement::sprite_movement)
        .add_systems(FixedUpdate, mouse_position_system)
        .add_systems(FixedUpdate, mouse_click_system)
        .add_systems(FixedUpdate, print_screen_size)
        .add_systems(FixedUpdate, projectile_movement_system)
        .add_systems(FixedUpdate, monitor_player_position_system)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
        
}

