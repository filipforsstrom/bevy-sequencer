use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::note::Note;

pub struct MouseInputPlugin;

impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
            .add_system(select_note)
            .add_system(move_note);
    }
}

#[derive(Resource, Debug)]
pub struct Selected {
    pub entity: Option<Entity>,
}

impl Default for Selected {
    fn default() -> Self {
        Selected { entity: None }
    }
}

// This system prints messages when you press or release the left mouse button:
fn select_note(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected: ResMut<Selected>,
    notes_query: Query<(Entity, &Transform), With<Note>>,
) {
    let mut cursor_position: Vec3 = Vec3::new(0., 0., 0.);

    for event in cursor_moved_events.iter() {
        cursor_position = Vec3::new(event.position.x, event.position.y, 0.0);
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (entity, transform) in notes_query.iter() {
            let collision = collide(
                transform.translation,
                transform.scale.truncate(),
                cursor_position,
                Vec2::new(1.0, 1.0),
            );

            if collision.is_some() {
                if mouse_button_input.pressed(MouseButton::Left) {
                    info!("selected note: {:?}", entity);
                    selected.entity = Some(entity);
                } else if mouse_button_input.just_released(MouseButton::Left) {
                    info!("deselected note: {:?}", entity);
                    selected.entity = None;
                }
            }
        }
    }
}

fn move_note(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    selected: Res<Selected>,
    mut note_query: Query<&mut Transform, With<Note>>,
) {
    if selected.entity.is_some() && mouse_button_input.pressed(MouseButton::Left) {
        if let Ok(mut transform) = note_query.get_mut(selected.entity.unwrap()) {
            info!("moving note");
            for event in cursor_moved_events.iter() {
                transform.translation.x = event.position.x;
                transform.translation.y = event.position.y;
            }
        }
    }
}
