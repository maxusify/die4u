use bevy::prelude::*;

use crate::core::GameState;

use self::systems::spawn_default_player;

mod bundle;
mod components;
mod systems;

/// Plugin that adds player to the game
pub struct MobPlayerPlugin;

impl Plugin for MobPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_default_player.in_schedule(OnEnter(GameState::Playing)));
    }
}
