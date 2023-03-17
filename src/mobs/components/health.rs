use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Health {
    pub current_health: u32,
    pub max_health: u32,
}
