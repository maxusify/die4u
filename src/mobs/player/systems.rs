use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{core::assets::PlayerAssets, mobs::mob::DefaultMobBundle};

use super::bundle::PlayerBundle;

/// Spawns default player
pub fn spawn_default_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands.spawn((
        PlayerBundle {
            mob: DefaultMobBundle {
                sprite: SpriteSheetBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, 10.0, 0.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: assets.sprite_idle.clone(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        // RigidBody
        RigidBody::Dynamic,
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        },
        // Collision
        Collider::cuboid(10.0, 30.0),
    ));
}
