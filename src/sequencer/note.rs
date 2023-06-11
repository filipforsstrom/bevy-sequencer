use bevy::{prelude::*, utils::HashMap, window::PrimaryWindow};
use rand::random;

const NUMBER_OF_RANDOM_NOTES: usize = 10;

pub struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_random_notes)
            .add_system(get_note_position)
            .add_system(note_pitch);
    }
}

#[derive(Component)]
pub struct Note {
    pub position: Vec2,
    pub pitch: u8,
}

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Notes {
    pub notes: HashMap<u8, bool>,
}

impl Default for Notes {
    fn default() -> Self {
        let mut notes = HashMap::default();

        for i in 0..128 {
            notes.insert(i, false);
        }

        Notes { notes }
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
                transform: Transform {
                    translation: Vec3::new(random_x, random_y, 0.0),
                    scale: Vec3::new(120.0, 20.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0., 1., 0.),
                    ..default()
                },
                ..default()
            })
            .insert(Note {
                position: Vec2::new(1.0, 0.0),
                pitch: 60,
            })
            .insert(Collider);
    }
}

pub fn get_note_position(mut note_query: Query<(&mut Note, &Transform), With<Note>>) {
    for (mut note, note_transform) in note_query.iter_mut() {
        note.position = Vec2::new(note_transform.translation.x, note_transform.translation.y);
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
