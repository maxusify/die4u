use crate::mobs::Player;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};

/// Marker component for main camera
#[derive(Component)]
pub struct MainCamera;

/// Aspect ratio for cameras
const ASPECT_RATIO: f32 = 16.0 / 9.0;

/// Setup main camera for the game
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

/// Make camera follow the player
pub fn _camera_follows_player(
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

/// Makes camera fit inside LDtk level boundaries
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            // Extract ldtk level from the handle
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                // Extract level itself
                let level = &ldtk_level.level;

                // If that level has index of 0
                if level_selection.is_match(&0, level) {
                    // Get level pixel width/height ratio
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    // Set origin for the camera
                    orthographic_projection.viewport_origin = Vec2::ZERO;

                    // Level is wider than the screen
                    if level_ratio > ASPECT_RATIO {
                        let height = (level.px_hei as f32 / 9.).round() * 9.;
                        let width = height * ASPECT_RATIO;

                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };

                        camera_transform.translation.x =
                            (player_translation.x - level_transform.translation.x - width / 2.)
                                .clamp(0., level.px_wid as f32 - width);

                        camera_transform.translation.y = 0.;
                    } else {
                        // Level is taller than the screen
                        let width = (level.px_wid as f32 / 16.).round() * 16.;
                        let height = width / ASPECT_RATIO;

                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };

                        camera_transform.translation.y =
                            (player_translation.y - level_transform.translation.y - height / 2.)
                                .clamp(0., level.px_hei as f32 - height);

                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}
