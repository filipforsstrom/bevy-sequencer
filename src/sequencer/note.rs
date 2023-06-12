use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::NUMBER_OF_RANDOM_PLAYHEADS;

use super::sequence::GlobalSequencerSettings;

const NUMBER_OF_RANDOM_NOTES: usize = 3;

pub struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_random_notes)
            .add_system(note_pitch);
    }
}

#[derive(Component)]
pub struct Note {
    pub position: Vec2,
    pub pitch: u8,
}

#[derive(Component)]
pub struct Collider {
    pub state: CollisionState,
}

pub enum CollisionState {
    NoCollision,
    CollisionStart,
    CollisionContinue,
    CollisionEnd,
}

impl Default for Collider {
    fn default() -> Self {
        Collider {
            state: CollisionState::NoCollision,
        }
    }
}

pub fn spawn_random_notes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let random_positions = (0..NUMBER_OF_RANDOM_NOTES)
        .map(|_| {
            Vec2::new(
                random::<f32>() * window.width(),
                random::<f32>() * window.height(),
            )
        })
        .collect::<Vec<Vec2>>();

    for playhead in 0..NUMBER_OF_RANDOM_PLAYHEADS {
        for note in 0..NUMBER_OF_RANDOM_NOTES {
            let random_x = random_positions[note].x;
            let random_y = random_positions[note].y;

            commands
                .spawn(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(random_x, random_y, playhead as f32),
                        scale: Vec3::new(120., 20., 0.),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(0., 1., 0.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Note {
                    position: Vec2::new(1., 0.),
                    pitch: 60,
                })
                .insert(Collider { ..default() });
        }
    }
}

pub fn note_pitch(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut note_query: Query<(&mut Note, &Transform), With<Note>>,
    sequencer_settings: Res<GlobalSequencerSettings>,
) {
    let window = window_query.get_single().unwrap();
    let window_min = 0. as f32;
    let window_max = window.height();
    let min = sequencer_settings.pitch_min;
    let max = sequencer_settings.pitch_max;

    for (mut note, note_transform) in note_query.iter_mut() {
        let note_y_position_as_midi = map_to_midi_range(
            note_transform.translation.y,
            window_min,
            window_max,
            min,
            max,
        );

        note.pitch = note_y_position_as_midi;
    }
}

fn map_to_midi_range(value: f32, old_min: f32, old_max: f32, new_min: u8, new_max: u8) -> u8 {
    let midi_value = ((value - old_min) * (new_max as f32 - new_min as f32)) / (old_max - old_min)
        + new_min as f32;
    midi_value.max(0.0).min(127.0) as u8
}
