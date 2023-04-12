use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

/// Player assets
mod assets;
/// Player bundle
mod bundle;
/// Player specific components
mod components;
/// Player specific resources
mod resources;
/// Player systems
mod systems;

use self::assets::PlayerAssets;
pub use self::components::Player;
use self::systems::{player_movement_with_physics, spawn_default_player_with_physics};

/// Plugin that adds player to the game
pub struct MobPlayerPlugin;

impl Plugin for MobPlayerPlugin {
    fn build(&self, app: &mut App) {
        // Dynamic assets collection for player
        app.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::AssetLoading,
            "player.assets.ron",
        )
        .add_collection_to_loading_state::<_, PlayerAssets>(GameState::AssetLoading);

        app.add_system(spawn_default_player_with_physics.in_schedule(OnEnter(GameState::Playing)));
        app.add_system((player_movement_with_physics).in_set(OnUpdate(GameState::Playing)));
    }
}
