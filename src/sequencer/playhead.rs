use bevy::{prelude::*, sprite::collide_aabb::collide, window::PrimaryWindow};
use rand::{random, Rng};

use crate::{sequencer::note::CollisionState, NUMBER_OF_RANDOM_PLAYHEADS};

use super::note::{Collider, Note, NoteOn};

const DEFAULT_PLAYHEAD_SPEED: f32 = 300.0;

pub struct PlayheadPlugin;

impl Plugin for PlayheadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NoteOnEvent>()
            .add_event::<NoteOffEvent>()
            .add_startup_system(spawn_random_playheads)
            .add_system(playhead_movement)
            .add_system(check_for_collisions)
            // .add_system(note_struck)
            .add_system(check_note_on);
    }
}

#[derive(Component)]
pub struct Playhead {
    pub direction: PlayheadDirection,
    pub current_direction: PlayheadDirection,
    pub speed: f32,
}

impl Default for Playhead {
    fn default() -> Self {
        Playhead {
            direction: PlayheadDirection::Right,
            current_direction: PlayheadDirection::Right,
            speed: DEFAULT_PLAYHEAD_SPEED,
        }
    }
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

pub struct NoteOnEvent(pub Entity);

pub struct NoteOffEvent(pub Entity);

// pub fn spawn_playhead(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
//     let window = window_query.get_single().unwrap();
//     let height = window.height();

//     // Rectangle
//     commands
//         .spawn(SpriteBundle {
//             transform: Transform {
//                 translation: Vec3::new(0., height / 2., 0.),
//                 scale: Vec3::new(5.0, height, 0.0),
//                 ..default()
//             },
//             sprite: Sprite {
//                 color: Color::rgb(1., 0., 0.),
//                 ..default()
//             },
//             ..default()
//         })
//         .insert(Playhead { ..default() });
// }

pub fn spawn_random_playheads(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let height = window.height();
    let mut rng = rand::thread_rng();
    let lower_bound = 100.;
    let upper_bound = 300.;

    for playhead in 0..NUMBER_OF_RANDOM_PLAYHEADS {
        let speed = rng.gen_range(lower_bound..=upper_bound);
        let z = playhead as f32;

        commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., height / 2., z),
                    scale: Vec3::new(5.0, height, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1., 0., 0.),
                    ..default()
                },
                ..default()
            })
            .insert(Playhead {
                speed: speed,
                ..default()
            });
    }
}

pub fn playhead_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut playhead_query: Query<(&mut Transform, &mut Playhead)>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, mut playhead) in playhead_query.iter_mut() {
        match &playhead.direction {
            PlayheadDirection::Right => {
                transform.translation.x += playhead.speed * time.delta_seconds();

                if transform.translation.x > window.width() {
                    transform.translation.x = 0.;
                }
            }
            PlayheadDirection::Left => {
                transform.translation.x -= playhead.speed * time.delta_seconds();

                if transform.translation.x > 0. {
                    transform.translation.x = window.width();
                }
            }
            PlayheadDirection::Pendulum => match &playhead.current_direction {
                PlayheadDirection::Right => {
                    transform.translation.x += playhead.speed * time.delta_seconds();

                    if transform.translation.x > window.width() {
                        playhead.current_direction = PlayheadDirection::Left;
                    }
                }
                PlayheadDirection::Left => {
                    transform.translation.x -= playhead.speed * time.delta_seconds();

                    if transform.translation.x < 0. {
                        playhead.current_direction = PlayheadDirection::Right;
                    }
                }
                PlayheadDirection::Pendulum => {}
            },
        }
    }
}

// pub fn note_struck(
//     playhead_query: Query<&Transform, With<Playhead>>,
//     note_query: Query<(Entity, &Transform), With<Note>>,
// ) {
//     if let Ok(playhead_transform) = playhead_query.get_single() {
//         for (note_entity, note_transform) in note_query.iter() {
//             if playhead_transform.translation.x > note_transform.translation.x - 5.0
//                 && playhead_transform.translation.x < note_transform.translation.x + 5.0
//             {
//                 // println!("{}", note_entity.index());
//                 // event_midi_out.send(NoteOnEvent(note_entity));
//             }
//         }
//     }
// }

pub fn check_for_collisions(
    mut midi_out_note_on: EventWriter<NoteOnEvent>,
    mut midi_out_note_off: EventWriter<NoteOffEvent>,
    playhead_query: Query<(Entity, &Transform, &Playhead)>,
    mut collider_query: Query<(Entity, &Transform, &mut Collider), With<Note>>,
) {
    for (playhead_entity, playhead_transform, playhead) in playhead_query.iter() {
        for (collider_entity, collider_transform, mut collider) in collider_query.iter_mut() {
            if playhead_transform.translation.z != collider_transform.translation.z {
                continue;
            }

            let collision = collide(
                playhead_transform.translation,
                playhead_transform.scale.truncate(),
                collider_transform.translation,
                collider_transform.scale.truncate(),
            );

            match collider.state {
                CollisionState::NoCollision => {
                    if collision.is_some() {
                        collider.state = CollisionState::CollisionStart;
                        midi_out_note_on.send(NoteOnEvent(collider_entity));
                    }
                }
                CollisionState::CollisionStart => {
                    if collision.is_some() {
                        collider.state = CollisionState::CollisionContinue;
                    } else {
                        midi_out_note_off.send(NoteOffEvent(collider_entity));
                        collider.state = CollisionState::CollisionEnd;
                    }
                }
                CollisionState::CollisionContinue => {
                    if collision.is_none() {
                        midi_out_note_off.send(NoteOffEvent(collider_entity));
                        collider.state = CollisionState::CollisionEnd;
                    }
                }
                CollisionState::CollisionEnd => {
                    if collision.is_none() {
                        collider.state = CollisionState::NoCollision;
                    }
                }
            }
        }
    }
}


pub fn check_note_on(
    mut midi_out_note_on: EventWriter<NoteOnEvent>,
    mut midi_out_note_off: EventWriter<NoteOffEvent>,
    note_query: Query<(Entity, &NoteOn), (Changed<NoteOn>, With<Note>)>,
) {
    for (entity, note) in note_query.iter() {
        if note.on {
            midi_out_note_on.send(NoteOnEvent(entity));
            // println!("Note on entity: {}.", entity.index());
        } else if !note.on {
            midi_out_note_off.send(NoteOffEvent(entity));
            // println!("Note off entity: {}.", entity.index());
        }
    }
}
