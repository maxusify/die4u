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
  input_map: Res<GamePlayerInput>
) {
    commands
        .spawn((
            PlayerBundle::default(),
            DefaultMobBundle {
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: assets.player_idle.clone(),
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
            ExternalForce::default(),
            ExternalImpulse::default(),
        ))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(10.0, 20.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)))
                .insert(ColliderMassProperties::Mass(3.0));
        });
}

/// Move player with physics
pub fn player_movement_with_physics(
  mut external_force: Query<&mut ExternalForce, With<Player>>,
  actions: Query<&ActionState<PlayerActions>, With<Player>>,
) {
    let mut old_force = external_force.single_mut();
    let mut new_force = Vec2::splat(0.0);
    let action_state = actions.single();

    // Process keys
    let _ = action_state
        .get_pressed()
        .iter()
        .map(|key| {
            match key {
                PlayerActions::Jump => new_force.y += 750.0,
                PlayerActions::MoveLeft => new_force.x -= 500.0,
                PlayerActions::MoveRight => new_force.x += 500.0,
                _ => (),
            }
        })
        .collect::<Vec<()>>();
    old_force.force = new_force;
}
