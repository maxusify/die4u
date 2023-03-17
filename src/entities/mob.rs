use bevy::prelude::*;

use crate::resources::GeneralAnimationTimer;

use super::components::{Health, InventoryBundle, Name, Shield};

/// Identifies that entity is a mob
#[derive(Component, Default)]
pub struct Mob;

/// Statistics
#[derive(Bundle, Default)]
pub struct StatisticsBundle {
    pub health: Health,
    pub shield: Shield,
}

/// Animation timer per entity
#[derive(Component, Default)]
pub struct AnimationTimer(pub Timer);

/// Bundle for creating basic mob with health and shield
#[derive(Bundle, Default)]
pub struct DefaultMobBundle {
    /// Marker component indicating that entity is a mob
    pub _mob: Mob,

    /// Name for the mob
    pub name: Name,

    /// Inventory mechanics
    #[bundle]
    pub inventory: InventoryBundle,

    /// Basic stats like: Health, Shield
    #[bundle]
    pub stats: StatisticsBundle,

    /// Bevy sprite sheet bundle
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

/// Animate sprites of mobs with their own timer
pub fn animate_mob_sprites(
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

/// Animate sprites of mobs without their own timer
pub fn animate_mob_sprites_global(
    timer: Res<GeneralAnimationTimer>,
    mut query: Query<&mut TextureAtlasSprite, Without<AnimationTimer>>,
) {
    for mut sprite in &mut query {
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 8;
        }
    }
}
