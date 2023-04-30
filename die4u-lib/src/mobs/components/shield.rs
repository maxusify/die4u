use bevy::prelude::*;

/// Shield points
#[derive(Clone, Component, Default)]
pub struct Shield {
    /// Current amount of shield points
    pub current_shield: u32,
    /// Maximum amount of shield points
    pub max_shield: u32,
}
