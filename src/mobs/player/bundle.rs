use bevy::prelude::*;

use crate::{
    core::animation::components::SpriteAnimationTimer,
    mobs::{components::Experience, mob::DefaultMobBundle},
};

use super::components::Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    /// Marker component indicating that entity is the player
    pub _player: Player,

    /// Experience mechanics bundle
    #[bundle]
    pub experience: Experience,

    /// Basic mob properties
    #[bundle]
    pub mob: DefaultMobBundle,

    /// Animation timer
    pub sprite_timer: SpriteAnimationTimer,
}
