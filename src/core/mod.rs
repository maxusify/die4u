use bevy::{app::PluginGroupBuilder, prelude::*};
///
/// Game input module
pub mod input;

/// Game startup systems
pub mod setup;

/// Game assets systems
pub mod assets;

/// All possible game states
#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
pub enum GameState {
    #[default]
    AssetLoading,
    MainMenu,
    SubMenu,
    Playing,
    Pause,
    GameOver,
}

/// Plugin group for all core game plugins
pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(setup::GameSetupPlugin)
            .add(assets::GameAssetsPlugin)
            .add(input::GameInputPlugin)
    }
}
