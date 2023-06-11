use bevy::prelude::*;
use bevy_midi::prelude::{MidiOutput, MidiOutputPlugin};

use super::{note::Note, playhead::{NoteOnEvent, NoteOffEvent}};

pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MidiOutputPlugin)
            .init_resource::<MidiSettings>()
            .add_system(connect)
            .add_system(midi_out_note_on)
            .add_system(midi_out_note_off);
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

fn midi_out_note_on(
    note_query: Query<&Note, With<Note>>,
    mut event_midi_out: EventReader<NoteOnEvent>,
    output: ResMut<MidiOutput>,
) {
    for ev in event_midi_out.iter() {
        if let Ok(note) = note_query.get(ev.0) {
            output.send([0b1001_0000, note.pitch, 127].into()); // Note on, channel 1
            println!("Midi note on: {}", note.pitch);
            // output.send([0b1001_0000, note.pitch, 0].into()); // Note off, channel 1
            // println!("Midi note off: {}", note.pitch);
        }
    }
}

fn midi_out_note_off(
    note_query: Query<&Note, With<Note>>,
    mut event_midi_out: EventReader<NoteOffEvent>,
    output: ResMut<MidiOutput>,
) {
    for ev in event_midi_out.iter() {
        if let Ok(note) = note_query.get(ev.0) {
            output.send([0b1001_0000, note.pitch, 0].into()); // Note off, channel 1
            println!("Midi note off: {}", note.pitch);
        }
    }
}
