use bevy::prelude::*;

/// Setup 2D Camera for the game
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
