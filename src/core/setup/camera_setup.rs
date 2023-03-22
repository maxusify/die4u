use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

/// Setup main camera for the game
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

/// System for camera follower
pub fn camera_follows_player() {
    todo!()
}
