use bevy::prelude::*;

use crate::core::assets::PlayerAssets;

use super::{
    components::Experience,
    mob::{AnimationTimer, DefaultMobBundle},
};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    /// Marker component indicating that entity is the player
    _player: Player,

    /// Experience mechanics bundle
    #[bundle]
    pub experience: Experience,

    /// Basic mob properties
    #[bundle]
    pub mob: DefaultMobBundle,

    /// Animation timer
    pub sprite_timer: AnimationTimer,
}

pub fn spawn_default_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands.spawn(PlayerBundle {
        mob: DefaultMobBundle {
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: assets.sprite_idle.clone(),
                ..Default::default()
            },
            name: super::components::Name("Player".to_string()),
            ..Default::default()
        },
        sprite_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ..Default::default()
    });
}
