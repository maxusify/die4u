use bevy::prelude::*;

/// Shield points
#[derive(Component, Default)]
pub struct Shield {
    pub current_shield: u32,
    pub max_shield: u32,
}
