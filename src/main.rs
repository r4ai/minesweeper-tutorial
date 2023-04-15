use bevy::prelude::*;
use bevy::window::WindowResolution;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use board_plugin::components::Coordinates;
use board_plugin::resources::BoardOptions;
use board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();

    // Add default plugins
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(700.0, 800.0),
            title: "Mine Sweeper!".to_string(),
            ..default()
        }),
        ..default()
    }));

    // Debug plugin
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_plugin(BoardPlugin);
    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        ..Default::default()
    });
    app.add_startup_system(camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
