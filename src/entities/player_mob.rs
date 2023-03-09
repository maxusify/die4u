use super::mob::{DefaultMobBundle, ExperienceLevels, ExperiencePoints};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub exp: ExperiencePoints,
    pub exp_level: ExperienceLevels,
    _player: Player,

    #[bundle]
    pub mob: DefaultMobBundle,
}
