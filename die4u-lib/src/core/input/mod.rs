use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

/// Gameplay related actions
mod gameplay;
/// Global input map
mod input_map;

pub use self::gameplay::player_actions::PlayerActions;
pub use self::input_map::GamePlayerInput;

/// Groups input related logic
pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerActions>::default());
        app.insert_resource(GamePlayerInput::default());
    }
}
