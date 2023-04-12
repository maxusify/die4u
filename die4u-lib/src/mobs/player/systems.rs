use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::InputManagerBundle;

use crate::core::{GamePlayerInput, PlayerActions};
use crate::mobs::mob::DefaultMobBundle;

use super::assets::PlayerAssets;
use super::bundle::PlayerBundle;
use super::components::Player;

/// Spawns default player as [`RigidBody`]
pub fn spawn_default_player_with_physics(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    input_map: Res<GamePlayerInput>,
) {
    commands
        .spawn((
            PlayerBundle::default(),
            DefaultMobBundle {
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: assets.idle.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.5),
                        scale: Vec3::splat(2.0),
                        ..default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            // Inputs
            InputManagerBundle::<PlayerActions> {
                input_map: input_map.input_map.clone(),
                ..default()
            },
            // Physics
            RigidBody::Dynamic,
            Velocity::default(),
            ExternalForce::default(),
            ExternalImpulse::default(),
        ))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(10.0, 20.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)))
                .insert(ColliderMassProperties::Mass(2.0));
        });
}

/// Move player with physics
pub fn player_movement_with_physics(
    mut external_force: Query<&mut ExternalForce, With<Player>>,
    mut player_query: Query<
        (
            &ActionState<PlayerActions>,
            &Velocity,
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
    player_assets: Res<PlayerAssets>,
) {
    let mut old_force = external_force.single_mut();
    let mut new_force = Vec2::splat(0.0);
    let (action_state, velocity, mut player_texture, mut player_atlas_sprite) =
        player_query.single_mut();

    // Process keys
    let _ = action_state
        .get_pressed()
        .iter()
        .map(|key| match key {
            PlayerActions::Jump => {
                new_force.y += 750.0;
            }
            PlayerActions::MoveLeft => {
                new_force.x -= 500.0;
                if velocity.linvel.x.abs() > 500.0 {
                    *player_atlas_sprite = player_assets.run.clone();
                } else {
                    *player_atlas_sprite = player_assets.walk.clone();
                }
                player_texture.flip_x = true;
            }
            PlayerActions::MoveRight => {
                new_force.x += 500.0;
                if velocity.linvel.x.abs() > 500.0 {
                    *player_atlas_sprite = player_assets.run.clone();
                } else {
                    *player_atlas_sprite = player_assets.walk.clone();
                }
                player_texture.flip_x = false;
            }
            _ => {
                *player_atlas_sprite = player_assets.idle.clone();
            }
        })
        .collect::<Vec<()>>();
    old_force.force = new_force;
}
