use bevy::prelude::Resource;

/// Player mob configuration
#[derive(Resource)]
pub struct PlayerConfig {
    /// Regular speed
    pub speed: f32,
    /// Maximum speed
    pub max_speed: f32,
}
