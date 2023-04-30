use crate::mobs::input::input_map::GamePlayerInput;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::InputManagerBundle;

use super::components::Player;
use super::input::player_actions::PlayerActions;
use crate::mobs::mob::DefaultMobBundle;

/// Bundle for spawning player entity
#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    /// Player is a mob
    #[bundle]
    pub mob: DefaultMobBundle,

    /// Input manager
    #[bundle]
    pub input: InputManagerBundle<PlayerActions>,

    /// Marker
    _player: Player,
}

impl Clone for PlayerBundle {
    fn clone(&self) -> Self {
        info!("Cloned PlayerBundle");
        Self {
            mob: self.mob.clone(),
            input: InputManagerBundle {
                action_state: self.input.action_state.clone(),
                input_map: self.input.input_map.clone(),
            },
            _player: self._player.clone(),
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        info!("Created default PlayerBundle");
        Self {
            input: InputManagerBundle::<PlayerActions> {
                input_map: GamePlayerInput::default().input_map,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
