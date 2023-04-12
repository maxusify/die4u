use bevy::prelude::*;

/// All possible game states
#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
pub enum GameState {
    #[default]
    /// Loading assets
    AssetLoading,
    /// Main menu
    MainMenu,
    /// Running gameplay
    Playing,
    /// Pause
    Pause,
    /// Game over
    GameOver,
}
