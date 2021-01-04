use bevy::{
    prelude::*,
    render::pass::ClearColor,
    // sprite::collide_aabb::{collide, Collision},
};

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let texture_handle = asset_server.load("sensei.png");
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}


/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        println!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        println!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        println!("'A' just released");
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(keyboard_input_system.system())
        .run();
}
