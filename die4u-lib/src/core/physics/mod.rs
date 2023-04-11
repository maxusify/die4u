use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePhysicsEnginePlugin;

impl Plugin for GamePhysicsEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0));
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}
