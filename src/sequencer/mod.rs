mod control_panel;
mod midi;
mod note;
mod playhead;

use control_panel::ControlPanelPlugin;
use midi::MidiPlugin;
use note::NotePlugin;
use playhead::PlayheadPlugin;

use bevy::prelude::*;

pub struct SequencerPlugin;

impl Plugin for SequencerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ControlPanelPlugin);
        app.add_plugin(MidiPlugin);
        app.add_plugin(PlayheadPlugin);
        app.add_plugin(NotePlugin);
    }
}
