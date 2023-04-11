use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PlayerConfig {
    pub speed: f32,
    pub max_speed: f32,
}
