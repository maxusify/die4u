use bevy::prelude::*;

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
