//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

#[derive(Component)]
struct PlayHead;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("branding/icon.png"),
    //         transform: Transform::from_xyz(100., 0., 0.),
    //         ..default()
    //     },
    //     Direction::Left,
    // ));

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Left => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x -= 150. * time.delta_seconds(),
        }

        if transform.translation.x > 200. {
            *logo = Direction::Right;
        } else if transform.translation.x < -200. {
            *logo = Direction::Left;
        }
    }
}