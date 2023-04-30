use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::InputMap;

use crate::GameState;
use crate::mobs::input::player_actions::PlayerActions;
use crate::mobs::player::systems::get_player_input_info;

/// Player assets
mod assets;
/// Player bundle
mod bundle;
/// Player specific components
mod components;
/// Input for player
pub mod input;
/// Player movement logic
mod movement;
/// Player specific resources
mod resources;
/// Player systems
mod systems;

pub use self::bundle::PlayerBundle;
pub use self::components::Player;

use self::assets::PlayerAssets;
use self::systems::player_movement_with_physics;

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

        // Player systems
        app.add_system(player_movement_with_physics.in_set(OnUpdate(GameState::Playing)));
        app.add_system(get_player_input_info.in_schedule(OnEnter(GameState::Playing)));
    }
}
