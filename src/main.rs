use bevy::log::LogPlugin;
use bevy::window::WindowResolution;
use bevy::{log, prelude::*};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::resources::{BoardAssets, BoardOptions, SpriteMaterial};
use board_plugin::BoardPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Out,
    InGame,
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
        .add_startup_system(setup_board)
        .add_startup_system(camera_setup)
        .run();
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.0,
        safe_start: true,
        ..Default::default()
    });

    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            color: Color::WHITE,
        },
    });

    // Plugin activation
    state.set(AppState::InGame);
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
