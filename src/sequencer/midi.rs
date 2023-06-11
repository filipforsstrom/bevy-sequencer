use bevy::prelude::*;
use bevy_midi::prelude::{MidiOutput, MidiOutputPlugin};

pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MidiOutputPlugin)
            .init_resource::<MidiSettings>()
            .add_system(connect);
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
