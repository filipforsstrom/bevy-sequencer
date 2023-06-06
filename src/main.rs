use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_midi::prelude::*;
use rand::random;

const PLAYHEAD_SPEED: f32 = 500.0;
const NUMBER_OF_RANDOM_NOTES: usize = 2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MidiOutputPlugin)
        .init_resource::<MidiSettings>()
        .add_system(ui_example_system)
        .add_system(connect)
        .add_event::<MidiOutEvent>()
        .add_system(midi_out)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_playhead)
        .add_startup_system(spawn_random_notes)
        .add_system(playhead_movement)
        .add_system(note_pitch)
        .add_system(note_struck)
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

pub struct MidiOutEvent(Entity);

fn midi_out(
    note_query: Query<&Note, With<Note>>,
    mut event_midi_out: EventReader<MidiOutEvent>,
    output: ResMut<MidiOutput>,
) {
    for ev in event_midi_out.iter() {
        if let Ok(note) = note_query.get(ev.0) {
            output.send([0b1001_0000, note.pitch, 127].into()); // Note on, channel 1, max velocity
            output.send([0b1000_0000, note.pitch, 127].into()); // Note off, channel 1, max velocity
            println!("Midi out: {:?}", note.pitch);
        }
    }
}
