use bevy::prelude::*;

use crate::{
    entities::player::{animate_sprites_system, spawn_default_player},
    resources::GameSettings,
};

use super::GameState;

/// Setup for 2D camera
pub mod camera_setup;

/// Window initial setup
pub mod window_setup;

/// Plugin grouping all setup systems for initializing game
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        // Add resource for game settings
        app.init_resource::<GameSettings>();
        // Add default main camera
        app.add_startup_system(camera_setup::setup_camera);

        // Temporary player setup
        app.add_system(spawn_default_player.in_schedule(OnEnter(GameState::Playing)));
        app.add_system(animate_sprites_system.run_if(in_state(GameState::Playing)));
    }
}
