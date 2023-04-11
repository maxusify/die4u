use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{InputMap, QwertyScanCode};
use leafwing_input_manager::Actionlike;

#[derive(Resource)]
pub struct GamePlayerInput {
    pub input_map: InputMap<PlayerActions>,
}

impl Default for GamePlayerInput {
    fn default() -> Self {
        let mut input_map = InputMap::default();

        input_map
            .insert(QwertyScanCode::W, PlayerActions::Jump)
            .insert(QwertyScanCode::S, PlayerActions::Fall)
            .insert(QwertyScanCode::A, PlayerActions::MoveLeft)
            .insert(QwertyScanCode::D, PlayerActions::MoveRight);

        Self { input_map }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerActions {
    // Movement
    MoveLeft,
    MoveRight,
    Jump,
    Fall,
}

pub struct GameInputPlugin;
impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerActions>::default());
    }
}
