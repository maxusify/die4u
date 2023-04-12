use crate::core::setup::camera_setup::camera_follows_player;
use crate::GameState;
use bevy::prelude::*;

use self::window_setup::initialize_window;

/// Setup for 2D camera
pub mod camera_setup;

/// Window initial setup
pub mod window_setup;

/// Plugin grouping all setup systems for initializing game
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        // Default Bevy plugins
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: initialize_window(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        );

        // Add default main camera
        app.add_startup_system(camera_setup::setup_camera);
        app.add_system(camera_follows_player.in_set(OnUpdate(GameState::Playing)));
    }
}
