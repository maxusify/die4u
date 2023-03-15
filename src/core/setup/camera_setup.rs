use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

/// Setup main camera for the game
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
