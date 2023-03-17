use bevy::{app::PluginGroupBuilder, prelude::*};

/// Game input module
mod input;

/// Game states
mod game_state;
pub use game_state::GameState;

/// Game startup systems
pub mod setup;

/// Game assets systems
pub mod assets;

/// Game animations systems
pub mod animation;

/// Game physics engine
pub mod physics;

/// Plugin group for all core game plugins
pub struct GameCorePluginsGroup;

impl PluginGroup for GameCorePluginsGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(setup::GameSetupPlugin)
            .add(assets::GameAssetsPlugin)
            .add(input::GameInputPlugin)
            .add(animation::GameAnimationsPlugin)
            .add(physics::GamePhysicsEnginePlugin)
    }
}
