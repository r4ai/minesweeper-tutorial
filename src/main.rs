use bevy::log::LogPlugin;
use bevy::window::WindowResolution;
use bevy::{log, prelude::*};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::resources::BoardOptions;
use board_plugin::BoardPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    Out,
}

fn main() {
    let mut app = App::new();

    // Add default plugins
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(700.0, 800.0),
                    title: "Mine Sweeper!".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                level: log::Level::DEBUG,
                ..default()
            }),
    );

    // Debug plugin
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_state::<AppState>()
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame,
        })
        .add_system(state_handler)
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 3.0,
            safe_start: true,
            ..Default::default()
        });
    app.add_startup_system(camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn state_handler(mut next_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        log::info!("clearing game");
        next_state.set(AppState::Out);
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        log::info!("loading game");
        next_state.set(AppState::InGame);
    }
}
