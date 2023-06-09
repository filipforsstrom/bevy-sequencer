use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::collide_aabb::collide,
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_midi::prelude::*;
use rand::random;

const PLAYHEAD_SPEED: f32 = 500.0;
const NUMBER_OF_RANDOM_NOTES: usize = 10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MidiOutputPlugin)
        .init_resource::<MidiSettings>()
        .init_resource::<Cartesian>()
        .add_system(ui_example_system)
        .add_system(connect)
        .add_event::<MidiOutEvent>()
        .add_system(midi_out)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_playhead)
        // .add_startup_system(spawn_random_notes)
        .add_system(playhead_movement)
        .add_system(note_pitch)
        .add_system(note_struck)
        .add_system(note_collision)
        .add_startup_system(load_image)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(sequencer_timer_setup)
        .add_system(tick)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts, output: Res<MidiOutput>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
        for (i, (name, _)) in output.ports().iter().enumerate() {
            ui.label(format!("Port {:?}: {:?}", i, name));
        }
    });
}

fn connect(output: Res<MidiOutput>, mut midi_settings: ResMut<MidiSettings>) {
    if midi_settings.connected {
        return;
    }

    if let Some((_, port)) = output.ports().get(0) {
        output.connect(port.clone());
        midi_settings.connected = true;
        println!("Connected");
    }
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
    pub direction: PlayheadDirection,
    pub current_direction: PlayheadDirection,
}

pub enum PlayheadDirection {
    Right,
    Left,
    Pendulum,
}

impl Default for PlayheadDirection {
    fn default() -> Self {
        PlayheadDirection::Right
    }
}

#[derive(Component)]
pub struct Note {
    pub position: Vec2,
    pub pitch: u8,
}

#[derive(Resource, Default, Debug)]
struct MidiSettings {
    connected: bool,
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
            direction: PlayheadDirection::Pendulum,
            current_direction: PlayheadDirection::Right,
        });
}

pub fn playhead_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut playhead_query: Query<(&mut Transform, &mut Playhead)>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, mut playhead) in playhead_query.iter_mut() {
        let position = Vec3::new(playhead.position.x, playhead.position.y, 0.0);

        match &playhead.direction {
            PlayheadDirection::Right => {
                transform.translation += position * PLAYHEAD_SPEED * time.delta_seconds();

                if transform.translation.x > window.width() {
                    transform.translation.x = 0.;
                }
            }
            PlayheadDirection::Left => {
                transform.translation -= position * PLAYHEAD_SPEED * time.delta_seconds();

                if transform.translation.x > 0. {
                    transform.translation.x = window.width();
                }
            }
            PlayheadDirection::Pendulum => match &playhead.current_direction {
                PlayheadDirection::Right => {
                    transform.translation += position * PLAYHEAD_SPEED * time.delta_seconds();

                    if transform.translation.x > window.width() {
                        playhead.current_direction = PlayheadDirection::Left;
                    }
                }
                PlayheadDirection::Left => {
                    transform.translation -= position * PLAYHEAD_SPEED * time.delta_seconds();

                    if transform.translation.x < 0. {
                        playhead.current_direction = PlayheadDirection::Right;
                    }
                }
                PlayheadDirection::Pendulum => {}
            },
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
                    custom_size: Some(Vec2::new(100., 10.)),
                    ..default()
                },
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            })
            .insert(Note {
                position: Vec2::new(1.0, 0.0),
                pitch: 60,
            });
    }
}

pub fn note_pitch(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut note_query: Query<(&mut Note, &Transform), With<Note>>,
) {
    let window = window_query.get_single().unwrap();

    for (mut note, note_transform) in note_query.iter_mut() {
        let note_y_position_as_midi =
            map_to_midi_range(note_transform.translation.y, 0., window.height(), 0, 127);

        note.pitch = note_y_position_as_midi;
    }
}

fn map_to_midi_range(value: f32, old_min: f32, old_max: f32, new_min: u8, new_max: u8) -> u8 {
    let midi_value = ((value - old_min) * (new_max as f32 - new_min as f32)) / (old_max - old_min)
        + new_min as f32;
    midi_value.max(0.0).min(127.0) as u8
}

pub fn note_struck(
    mut event_midi_out: EventWriter<MidiOutEvent>,
    playhead_query: Query<&Transform, With<Playhead>>,
    note_query: Query<(Entity, &Transform), With<Note>>,
) {
    if let Ok(playhead_transform) = playhead_query.get_single() {
        for (note_entity, note_transform) in note_query.iter() {
            if playhead_transform.translation.x > note_transform.translation.x - 5.0
                && playhead_transform.translation.x < note_transform.translation.x + 5.0
            {
                // println!("{}", note_entity.index());
                event_midi_out.send(MidiOutEvent(note_entity));
            }
        }
    }
}

pub fn note_collision(
    mut commands: Commands,
    mut note_query: Query<(Entity, &mut Note, &Transform), With<Note>>,
    playhead_query: Query<&Transform, With<Playhead>>,
) {
    if let Ok(playhead_transform) = playhead_query.get_single() {
        for (note_entity, mut note, note_transform) in note_query.iter_mut() {
            let collision = collide(
                playhead_transform.translation,
                Vec2::new(5.0, 5.0),
                note_transform.translation,
                Vec2::new(100.0, 10.0),
            );
            if collision.is_some() {
                println!("Collision!");
                // commands.entity(note_entity).despawn();
            }
        }
    }
}

pub struct MidiOutEvent(Entity);

fn midi_out(
    note_query: Query<&Note, With<Note>>,
    mut event_midi_out: EventReader<MidiOutEvent>,
    output: ResMut<MidiOutput>,
) {
    for ev in event_midi_out.iter() {
        if let Ok(note) = note_query.get(ev.0) {
            output.send([0b1001_0000, note.pitch, 127].into()); // Note on, channel 1
            println!("Midi note on: {}", note.pitch);
            output.send([0b1001_0000, note.pitch, 0].into()); // Note off, channel 1
            println!("Midi note off: {}", note.pitch);
        }
    }
}

fn load_image(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(SpriteBundle {
        texture: asset_server.load("images/1234.png"),
        transform: Transform::from_translation(Vec3::new(
            window.width() / 2.,
            window.height() / 2.,
            0.,
        )),
        ..default()
    });
}

#[derive(Resource, Default, Debug)]
struct Cartesian {
    position: Vec<(i32, i32)>,
    size: usize,
}

fn cartesian_setup(mut cartesian_settings: ResMut<Cartesian>) {
    cartesian_settings.size = 4;
    cartesian_settings.position = vec![(0, 0); cartesian_settings.size];
}

use std::time::Duration;

#[derive(Resource)]
struct SequencerTimer {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

fn tick(mut commands: Commands, time: Res<Time>, mut config: ResMut<SequencerTimer>) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        println!("Tick");
    }
}

/// Configure our bomb spawning algorithm
fn sequencer_timer_setup(mut commands: Commands) {
    commands.insert_resource(SequencerTimer {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}
