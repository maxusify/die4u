use bevy::prelude::*;

use crate::{core::animation::components::SpriteAnimationTimer, mobs::components::Experience};

use super::components::Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    /// Marker component indicating that entity is the player
    pub _player: Player,

    /// Experience mechanics bundle
    #[bundle]
    pub experience: Experience,

    /// Animation timer
    pub sprite_timer: SpriteAnimationTimer,
}
