use bevy::prelude::*;
use bevy::time::Timer;

/// Animation timer for enitties (game ctx) that does not have their own timers
#[derive(Resource)]
pub struct GlobalSpriteAnimationTimer(pub Timer);
