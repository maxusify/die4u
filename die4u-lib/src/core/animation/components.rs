use bevy::prelude::*;

/// Timer for animating individual sprites
#[derive(Component, Clone)]
pub struct SpriteAnimationTimer(pub Timer);

impl Default for SpriteAnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}
