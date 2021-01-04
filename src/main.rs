use bevy::{
    prelude::*,
    render::pass::ClearColor,
    // sprite::collide_aabb::{collide, Collision},
};

struct ScoreBoard {
    points: usize,
}
struct Sensei {
    health: usize,
    speed: f32,
}

enum Collider {
    Solid,
    Player,   
}

fn setup(
    commands: &mut Commands, 
    asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let texture_handle = asset_server.load("sensei.png");
    commands
        .spawn(Camera2dBundle::default())
        
        // Sensei
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0,-215.0, 0.0)),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with( Sensei { health: 100, speed: 200.0 })
        .with(Collider::Player);
}

fn sensei_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Sensei, &mut Transform)>,
) {
    for (sensei, mut transform) in query.iter_mut() {
        let mut x_direction = 0.0;
        let mut y_direction = 0.0;
        
        if keyboard_input.pressed(KeyCode::Left) {
            x_direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            x_direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            y_direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            y_direction -= 1.0;
        }

        let translation = &mut transform.translation;
        // move sensei horizontally
        translation.x += time.delta_seconds() * x_direction * sensei.speed;
        // move sensei vertically
        translation.y += time.delta_seconds() * y_direction * sensei.speed;
            
        // bound sensei within the walls
        translation.x = translation.x.min(380.0).max(-380.0);
        translation.y = translation.y.min(300.0).max(-300.0);
    }
}
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ScoreBoard { points: 0 })
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(sensei_movement_system.system())
        .run();
}
