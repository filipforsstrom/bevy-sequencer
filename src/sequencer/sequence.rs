use bevy::prelude::*;

pub struct SequencePlugin;

impl Plugin for SequencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalSequencerSettings>();
    }
}

#[derive(Resource, Debug)]
pub struct GlobalSequencerSettings {
    pub pitch_min: u8,
    pub pitch_max: u8,
}

impl Default for GlobalSequencerSettings {
    fn default() -> Self {
        GlobalSequencerSettings {
            pitch_min: 40,
            pitch_max: 110,
        }
    }
}
