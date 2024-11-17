use bevy::prelude::*;

#[derive(Component)]
struct Brick;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(1.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let rect_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-500.0, 300.0, 0.0),
            scale: Vec3::new(100.0, 100.0, 1.0),
            ..default()
        },
        ..default()
    };
    commands.spawn((rect_bundle, Brick));
}