use bevy::prelude::*;
use bevy_midi::prelude::{MidiOutput, MidiOutputPlugin};

use super::{playhead::MidiOutEvent, note::Note};

pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MidiOutputPlugin)
            .init_resource::<MidiSettings>()
            .add_system(connect)
            .add_system(midi_out);
    }
}

#[derive(Resource, Default, Debug)]
struct MidiSettings {
    connected: bool,
}

fn connect(output: Res<MidiOutput>, mut midi_settings: ResMut<MidiSettings>) {
    if midi_settings.connected {
        return;
    }

    if let Some((_, port)) = output.ports().get(0) {
        output.connect(port.clone());
        midi_settings.connected = true;
        println!("Connected");
    }
}

fn midi_out(
    note_query: Query<&Note, With<Note>>,
    mut event_midi_out: EventReader<MidiOutEvent>,
    output: ResMut<MidiOutput>,
) {
    for ev in event_midi_out.iter() {
        if let Ok(note) = note_query.get(ev.0) {
            output.send([0b1001_0000, note.pitch, 127].into()); // Note on, channel 1
            println!("Midi note on: {}", note.pitch);
            output.send([0b1001_0000, note.pitch, 0].into()); // Note off, channel 1
            println!("Midi note off: {}", note.pitch);
        }
    }
}
