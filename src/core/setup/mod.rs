use bevy::prelude::*;

use crate::{
    entities::{
        mob::{animate_mob_sprites, animate_mob_sprites_global},
        player::spawn_default_player,
    },
    resources::{tick_global_animation_timer, GameSettings, GeneralAnimationTimer},
};

use super::GameState;

/// Setup for 2D camera
pub mod camera_setup;

/// Window initial setup
pub mod window_setup;

/// Plugin grouping all setup systems for initializing game
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        // Add resource for game settings
        app.init_resource::<GameSettings>();
        // Global animation timer
        app.insert_resource(GeneralAnimationTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )));
        // Add default main camera
        app.add_startup_system(camera_setup::setup_camera);

        // Global animation timer
        app.add_system(tick_global_animation_timer.run_if(in_state(GameState::Playing)));
        app.add_system(animate_mob_sprites_global.in_schedule(OnEnter(GameState::Playing)));

        // Temporary player setup
        app.add_system(animate_mob_sprites.run_if(in_state(GameState::Playing)));
        app.add_system(spawn_default_player.in_schedule(OnEnter(GameState::Playing)));
    }
}
