use bevy::prelude::Bundle;

use super::components::Player;
use crate::core::animation::components::SpriteAnimationTimer;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    /// Marker component indicating that entity is the player
    pub _player: Player,

    /// Animation timer
    pub sprite_timer: SpriteAnimationTimer,
}
