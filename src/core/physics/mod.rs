use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::GameState;

pub struct GamePhysicsEnginePlugin;

impl Plugin for GamePhysicsEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0));
        app.add_plugin(RapierDebugRenderPlugin::default());
        app.add_system(setup_physics.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn setup_physics(mut commands: Commands) {
    // Create ground
    commands
        .spawn(Collider::cuboid(128.0, 32.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -64.0, 0.0)));

    // Bouncing ball
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(32.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 256.0, 0.0)));
}
