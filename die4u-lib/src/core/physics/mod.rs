use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Groups game physics engine core plugins and settings
pub struct GamePhysicsEnginePlugin;

impl Plugin for GamePhysicsEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0));
        app.add_plugin(RapierDebugRenderPlugin::default());
        app.insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..Default::default()
        });
    }
}
