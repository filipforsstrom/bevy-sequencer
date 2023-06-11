mod midi;

use midi::MidiPlugin;

use bevy::prelude::*;

pub struct SequencerPlugin;

impl Plugin for SequencerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MidiPlugin);
    }
}
