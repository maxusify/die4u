use bevy::prelude::*;
use leafwing_input_manager::prelude::{ActionState, InputMap};
use leafwing_input_manager::Actionlike;

/// Bundle grouping input related components
#[derive(Clone, Bundle)]
pub struct PlayerInputManagerBundle<T: Actionlike> {
    /// Action state of the player
    pub action_state: ActionState<T>,
    /// Input map of the player
    pub input_map: InputMap<T>,
}

impl<T: Actionlike> Default for PlayerInputManagerBundle<T> {
    fn default() -> Self {
        info!("Created player input manager.");
        Self {
            action_state: ActionState::<T>::default(),
            input_map: InputMap::<T>::default(),
        }
    }
}
