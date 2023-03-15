use bevy::prelude::*;

use crate::core::assets::PlayerAssets;

use super::{components::Experience, mob::DefaultMobBundle};

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct AnimationTimer(Timer);

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

pub fn animate_sprites_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 8;
        }
    }
}
