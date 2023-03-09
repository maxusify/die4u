use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::entities::mob::{spawn_basic_mob, welcome_basic_mob};

/// Setup 2D Camera for the game
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// All possible game states
#[derive(States, PartialEq, Eq, Debug, Default, Hash, Clone)]
pub enum GameState {
    #[default]
    AssetLoading,
    MainMenu,
    Playing,
    Pause,
    GameOver,
}

/// Plugin grouping all setup systems for initializing game
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((setup_camera, spawn_basic_mob))
            .add_system(welcome_basic_mob);
    }
}

/// Plugin group for all core game plugins
pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(GameSetupPlugin)
    }
}
