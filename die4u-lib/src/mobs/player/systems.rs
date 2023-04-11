use crate::core::PlayerActions;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::InputManagerBundle;

use super::assets::PlayerAssets;
use super::bundle::PlayerBundle;
use super::components::Player;
use crate::mobs::mob::DefaultMobBundle;

pub fn spawn_default_player_with_physics(mut commands: Commands, assets: Res<PlayerAssets>) {
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
            InputManagerBundle::<PlayerActions>::default(),
            // Physics
            RigidBody::Dynamic,
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

pub fn player_movement_with_physics(
    mut player: Query<&mut ExternalForce, With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    for mut apply_force in player.iter_mut() {
        let mut new_force = Vec2::splat(0.0);

        // Process keys
        let _: Vec<_> = keys
            .get_pressed()
            .map(|key| match key {
                KeyCode::W => new_force.y += 500.0,
                KeyCode::A => new_force.x -= 1000.0,
                KeyCode::D => new_force.x += 1000.0,
                _ => (),
            })
            .collect();

        apply_force.force = new_force;
    }
}
