use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use self::assets::PlayerAssets;
use self::systems::{player_movement_with_physics, spawn_default_player_with_physics};
use crate::GameState;

mod assets;
mod bundle;
mod components;
mod resources;
mod systems;

/// Plugin that adds player to the game
pub struct MobPlayerPlugin;

impl Plugin for MobPlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add assets collection to AssetLoading state
        // Dynamic assets collection
        app.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::AssetLoading,
            "player.assets.ron",
        )
        .add_collection_to_loading_state::<_, PlayerAssets>(GameState::AssetLoading);

        app.add_system(spawn_default_player_with_physics.in_schedule(OnEnter(GameState::Playing)));
        app.add_system(player_movement_with_physics);
    }
}
