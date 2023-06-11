use bevy::{prelude::*, window::PrimaryWindow, sprite::collide_aabb::collide};

use super::note::{Note, Collider};

const PLAYHEAD_SPEED: f32 = 500.0;

pub struct PlayheadPlugin;

impl Plugin for PlayheadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MidiOutEvent>()
            .add_startup_system(spawn_playhead)
            .add_system(playhead_movement)
            .add_system(check_for_collisions)
            .add_system(note_struck);
    }
}

#[derive(Component)]
pub struct Playhead {
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

pub struct MidiOutEvent(pub Entity);

pub fn spawn_playhead(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let height = window.height();

    // Rectangle
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., height / 2., 0.),
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
        match &playhead.direction {
            PlayheadDirection::Right => {
                transform.translation.x += PLAYHEAD_SPEED * time.delta_seconds();

                if transform.translation.x > window.width() {
                    transform.translation.x = 0.;
                }
            }
            PlayheadDirection::Left => {
                transform.translation.x -= PLAYHEAD_SPEED * time.delta_seconds();

                if transform.translation.x > 0. {
                    transform.translation.x = window.width();
                }
            }
            PlayheadDirection::Pendulum => match &playhead.current_direction {
                PlayheadDirection::Right => {
                    transform.translation.x += PLAYHEAD_SPEED * time.delta_seconds();

                    if transform.translation.x > window.width() {
                        playhead.current_direction = PlayheadDirection::Left;
                    }
                }
                PlayheadDirection::Left => {
                    transform.translation.x -= PLAYHEAD_SPEED * time.delta_seconds();

                    if transform.translation.x < 0. {
                        playhead.current_direction = PlayheadDirection::Right;
                    }
                }
                PlayheadDirection::Pendulum => {}
            },
        }
    }
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

fn check_for_collisions(
    mut commands: Commands,
    playheads_query: Query<(Entity, &Transform), With<Playhead>>,
    collider_query: Query<(Entity, &Transform, Option<&Note>), With<Collider>>,
) {
    // Loop through all the projectiles on screen
    for (playhead_entity, playhead_transform) in &playheads_query {
        // Loop through all collidable elements on the screen
        // TODO: Figure out how to flatten this - 2 for loops no bueno
        for (collider_entity, collider_transform, enemy_check) in &collider_query {
            let collision = collide(
                playhead_transform.translation,
                playhead_transform.scale.truncate(),
                collider_transform.translation,
                collider_transform.scale.truncate(),
            );

            if let Some(collision) = collision {
                println!("Collision!");
            }
        }
    }
}
