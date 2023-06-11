use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_midi::prelude::MidiOutput;

pub struct ControlPanelPlugin;

impl Plugin for ControlPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(ui_example_system);
    }
}

fn ui_example_system(mut contexts: EguiContexts, output: Res<MidiOutput>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
        for (i, (name, _)) in output.ports().iter().enumerate() {
            ui.label(format!("Port {:?}: {:?}", i, name));
        }
    });
}
