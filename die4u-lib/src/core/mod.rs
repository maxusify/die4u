use bevy::{app::PluginGroupBuilder, prelude::*};

/// Game animations logic
pub mod animation;
/// Game assets logic
mod assets;
/// Game audio engine logic
mod audio;
/// Game debugging utilities
mod debug;
/// Game states
mod game_state;
/// Game input logic
mod input;
/// Game physics logic
mod physics;
/// Game startup logic
mod setup;

pub use game_state::GameState;
pub use setup::camera_setup::MainCamera;

/// Plugin group for all core game plugins
pub struct GameCorePluginsGroup;

impl PluginGroup for GameCorePluginsGroup {
    fn build(self) -> PluginGroupBuilder {
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
