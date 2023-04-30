use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::mobs::input::player_actions::PlayerActions;

/// Groups input related logic
pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerActions>::default());
    }
}
