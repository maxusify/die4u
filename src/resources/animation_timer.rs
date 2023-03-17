use bevy::prelude::*;
use bevy::time::Timer;

/// Animation timer for enitties (game ctx) that does not have their own timers
#[derive(Resource)]
pub struct GeneralAnimationTimer(pub Timer);

pub fn tick_global_animation_timer(mut timer: ResMut<GeneralAnimationTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}
