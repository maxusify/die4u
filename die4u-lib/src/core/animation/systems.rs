use bevy::prelude::*;

use super::{components::SpriteAnimationTimer, resources::GlobalSpriteAnimationTimer};

/// Ticks global animation timer
pub fn tick_global_animation_timer(mut timer: ResMut<GlobalSpriteAnimationTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

/// Animate sprites of mobs with their own timer
pub fn animate_mob_sprites(
    time: Res<Time>,
    mut query: Query<(&mut SpriteAnimationTimer, &mut TextureAtlasSprite)>,
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
    timer: Res<GlobalSpriteAnimationTimer>,
    mut query: Query<&mut TextureAtlasSprite, Without<SpriteAnimationTimer>>,
) {
    for mut sprite in &mut query {
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 8;
        }
    }
}
