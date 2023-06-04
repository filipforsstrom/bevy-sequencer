//! Renders a 2D scene containing a single, moving sprite.

use bevy::{prelude::*, window::PrimaryWindow};

const PLAYHEAD_SPEED: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_playhead)
        .add_system(playhead_movement)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    // let windows = window_query.get_single().unwrap();

    // commands.spawn(Camera2dBundle {
    //     transform: Transform::from_xyz(windows.width() / 2.0, windows.height() / 2.0, 100.0),
    //     ..default()
    // });

    // commands.spawn(Camera2dBundle::default());

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..default()
    });
}

#[derive(Component)]
pub struct Playhead {
    pub position: Vec2,
}

pub fn spawn_playhead(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Rectangle
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0., 0.),
                custom_size: Some(Vec2::new(5.0, window.height())),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(window.width() / -2., 0., 0.)),
            ..Default::default()
        })
        .insert(Playhead {
            position: Vec2::new(1.0, 0.0),
        });
}

pub fn playhead_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut playhead_query: Query<(&mut Transform, &mut Playhead)>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, playhead) in playhead_query.iter_mut() {
        let position = Vec3::new(playhead.position.x, playhead.position.y, 0.0);
        transform.translation += position * PLAYHEAD_SPEED * time.delta_seconds();

        if transform.translation.x > window.width() / 2. {
            transform.translation.x = window.width() / -2.;
        }
    }
}
