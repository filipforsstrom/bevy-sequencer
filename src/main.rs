mod sequencer;

use sequencer::SequencerPlugin;

use bevy::{prelude::*, window::PrimaryWindow};

const NUMBER_OF_RANDOM_PLAYHEADS: usize = 3;
const GRID_SIZE_X: usize = 4;
const GRID_SIZE_Y: usize = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(SequencerPlugin)
        .init_resource::<Cartesian>()
        .add_startup_system(spawn_camera)
        .add_startup_system(load_assets)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_startup_system(cartesian_setup)
        // .add_startup_system(sequencer_timer_setup)
        // .add_system(tick)
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Running,
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let windows = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(windows.width() / 2.0, windows.height() / 2.0, 100.0),
        ..default()
    });
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Image> = asset_server.load("images/1234.png");
}

#[derive(Resource, Default, Debug)]
struct Cartesian {
    position: Vec<(i32, i32)>,
    size: usize,
}

fn cartesian_setup(mut cartesian_settings: ResMut<Cartesian>) {
    cartesian_settings.size = 1;
    cartesian_settings.position = vec![(0, 0)];
}

use std::time::Duration;

#[derive(Resource)]
struct SequencerTimer {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

fn tick(
    time: Res<Time>,
    mut config: ResMut<SequencerTimer>,
    mut cartesian_settings: ResMut<Cartesian>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        cartesian_settings.position[0].0 += 1;
        cartesian_settings.position[0].1 += 1;
        if cartesian_settings.position[0].0 >= cartesian_settings.size as i32 {
            cartesian_settings.position[0].0 = 0;
        }
        if cartesian_settings.position[0].1 >= cartesian_settings.size as i32 {
            cartesian_settings.position[0].1 = 0;
        }
        println!("{:?}", cartesian_settings.position);
    }
}

fn sequencer_timer_setup(mut commands: Commands) {
    commands.insert_resource(SequencerTimer {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}
