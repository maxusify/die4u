use bevy::prelude::*;

use crate::{core::assets::PlayerAssets, mobs::mob::DefaultMobBundle};

use super::bundle::PlayerBundle;

/// Spawns default player
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
            ..Default::default()
        },
        ..Default::default()
    });
}
