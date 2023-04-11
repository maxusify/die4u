use bevy::{app::PluginGroupBuilder, prelude::*};

/// Game states
mod game_state;
pub use game_state::GameState;
/// Game animations systems
pub mod animation;
/// Game assets systems
mod assets;
/// Game audio engine
mod audio;
/// Game debugging utilities
mod debug;
/// Game input system
mod input;
/// Game physics engine
mod physics;
/// Game startup systems
mod setup;
pub use input::PlayerActions;

/// Plugin group for all core game plugins
pub struct GameCorePluginsGroup;

impl PluginGroup for GameCorePluginsGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(setup::GameSetupPlugin)
            .add(assets::GameAssetsPlugin)
            .add(animation::GameAnimationsPlugin)
            .add(physics::GamePhysicsEnginePlugin)
            .add(audio::GameAudioPlugin)
            .add(debug::GameDebugPlugin)
            .add(input::GameInputPlugin)
    }
}
