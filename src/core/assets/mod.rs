use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

/// Module containing assets progress tracking systems
pub mod progress;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "images/player.png")]
    pub sprite: Handle<Image>,
}

/// Plugin grouping all assets related stuff
pub struct GameAssetsPlugin;
impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        // Add plugin for tracking progress
        app.add_plugin(
            ProgressPlugin::new(super::GameState::AssetLoading)
                .continue_to(super::GameState::Playing),
        );

        // Add assets collection to AssetLoading state
        app.add_collection_to_loading_state::<_, PlayerAssets>(super::GameState::AssetLoading);

        // App assets loading progress tracking
        app.add_system(progress::print_progress);
    }
}
