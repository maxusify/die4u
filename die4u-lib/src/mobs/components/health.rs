use bevy::prelude::*;

/// Health component
#[derive(Clone, Component, Default)]
pub struct Health {
    /// Current health of the mob
    pub current_health: u32,
    /// Max health of the mob
    pub max_health: u32,
}
