use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::random;

const PLAYHEAD_SPEED: f32 = 500.0;
const NUMBER_OF_RANDOM_NOTES: usize = 2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui_example_system)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_playhead)
        .add_startup_system(spawn_random_notes)
        .add_system(playhead_movement)
        .add_system(note_struck)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let windows = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(windows.width() / 2.0, windows.height() / 2.0, 100.0),
        ..default()
    });
}

#[derive(Component)]
pub struct Playhead {
    pub position: Vec2,
}

#[derive(Component)]
pub struct Note {
    pub position: Vec2,
}

pub fn spawn_playhead(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    // Rectangle
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0., 0.),
                custom_size: Some(Vec2::new(5.0, window.height() * 2.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
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

        if transform.translation.x > window.width() {
            transform.translation.x = 0.;
        }
    }
}

pub fn spawn_random_notes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_RANDOM_NOTES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 1., 0.),
                    custom_size: Some(Vec2::new(10., 10.)),
                    ..default()
                },
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            })
            .insert(Note {
                position: Vec2::new(1.0, 0.0),
            });
    }
}

pub fn note_struck(
    playhead_query: Query<&Transform, With<Playhead>>,
    note_query: Query<(Entity, &Transform), With<Note>>,
) {
    if let Ok(playhead_transform) = playhead_query.get_single() {
        for (note_entity, note_transform) in note_query.iter() {
            if playhead_transform.translation.x > note_transform.translation.x - 5.0
                && playhead_transform.translation.x < note_transform.translation.x + 5.0
            {
                println!("{}", note_entity.index());
            }
        }
    }
}
