use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {}

impl Default for GameSettings {
    fn default() -> Self {
        Self {}
    }
}
