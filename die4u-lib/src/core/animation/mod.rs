use bevy::prelude::*;

use self::{
    resources::GlobalSpriteAnimationTimer,
    systems::{animate_mob_sprites, animate_mob_sprites_global, tick_global_animation_timer},
};
use super::GameState;

/// Components for animations
pub mod components;
/// Global resources for animations
pub mod resources;
/// Logic for animations
mod systems;

/// Groups logic for game animations
pub struct GameAnimationsPlugin;
impl Plugin for GameAnimationsPlugin {
    fn build(&self, app: &mut App) {
        // Global animation timer
        app.insert_resource(GlobalSpriteAnimationTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )));
        app.add_system(tick_global_animation_timer);
        app.add_system(animate_mob_sprites_global.in_schedule(OnEnter(GameState::Playing)));
        app.add_system(animate_mob_sprites.run_if(in_state(GameState::Playing)));
    }
}
