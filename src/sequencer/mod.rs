mod midi;
mod control_panel;

use control_panel::ControlPanelPlugin;
use midi::MidiPlugin;


use bevy::prelude::*;

pub struct SequencerPlugin;

impl Plugin for SequencerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ControlPanelPlugin);
        app.add_plugin(MidiPlugin);
    }
}
