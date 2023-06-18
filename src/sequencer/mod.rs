mod control_panel;
mod midi;
mod mouse_input;
mod note;
mod playhead;
mod sequence;

use control_panel::ControlPanelPlugin;
use midi::MidiPlugin;
use mouse_input::MouseInputPlugin;
use note::NotePlugin;
use playhead::PlayheadPlugin;
use sequence::SequencePlugin;

use bevy::prelude::*;

pub struct SequencerPlugin;

impl Plugin for SequencerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugin(ControlPanelPlugin);
        app.add_plugin(MidiPlugin);
        app.add_plugin(PlayheadPlugin);
        app.add_plugin(NotePlugin);
        app.add_plugin(SequencePlugin);
        app.add_plugin(MouseInputPlugin);
    }
}
