use crate::mobs::Player;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

/// Setup main camera for the game
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

/// Make camera follow the player
pub fn camera_follows_player(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    for mut camera in camera_query.iter_mut() {
        for player in player_query.iter() {
            let player_position = player.translation.xy();
            camera.translation =
                Vec3::new(player_position.x, player_position.y, camera.translation.z);
        }
    }
}
