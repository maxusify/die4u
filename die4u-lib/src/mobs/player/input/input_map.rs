use bevy::prelude::*;
use leafwing_input_manager::prelude::{InputMap, QwertyScanCode};

use super::player_actions::PlayerActions;

/// Resource containing player input map
#[derive(Resource)]
pub struct GamePlayerInput {
    /// Input map for player actions
    pub input_map: InputMap<PlayerActions>,
}

impl Default for GamePlayerInput {
    fn default() -> Self {
        let mut input_map = InputMap::default();

        input_map
            .insert(QwertyScanCode::W, PlayerActions::Jump)
            .insert(QwertyScanCode::S, PlayerActions::Fall)
            .insert(QwertyScanCode::A, PlayerActions::MoveLeft)
            .insert(QwertyScanCode::D, PlayerActions::MoveRight)
            .insert(MouseButton::Left, PlayerActions::Extract)
            .insert(MouseButton::Right, PlayerActions::Place)
            .insert(KeyCode::Space, PlayerActions::Jump);

        info!("Created default input map for the player.");

        Self { input_map }
    }
}
