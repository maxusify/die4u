use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

/// Module containing assets progress tracking systems
pub mod progress;

/// Plugin grouping all assets related stuff
pub struct GameAssetsPlugin;
impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        // Loading state for assets
        app.add_loading_state(LoadingState::new(GameState::AssetLoading));

        // Add plugin for tracking progress
        app.add_plugin(
            ProgressPlugin::new(super::GameState::AssetLoading)
                .continue_to(super::GameState::Playing),
        );

        // App assets loading progress tracking
        app.add_system(progress::print_progress);
    }
}
