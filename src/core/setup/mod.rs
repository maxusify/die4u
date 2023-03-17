use bevy::prelude::*;

/// Setup for 2D camera
pub mod camera_setup;

/// Window initial setup
pub mod window_setup;

/// Plugin grouping all setup systems for initializing game
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        // Add default main camera
        app.add_startup_system(camera_setup::setup_camera);
    }
}
