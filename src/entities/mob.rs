use bevy::prelude::*;

use super::components::{Health, InventoryBundle, Name, Shield};

/// Identifies that entity is a mob
#[derive(Component, Default)]
pub struct Mob;

/// Statistics
#[derive(Bundle, Default)]
pub struct StatisticsBundle {
    pub health: Health,
    pub shield: Shield,
}

/// Bundle for creating basic mob with health and shield
#[derive(Bundle, Default)]
pub struct DefaultMobBundle {
    /// Marker component indicating that entity is a mob
    pub _mob: Mob,

    /// Name for the mob
    pub name: Name,

    /// Inventory mechanics
    #[bundle]
    pub inventory: InventoryBundle,

    /// Basic stats like: Health, Shield
    #[bundle]
    pub stats: StatisticsBundle,

    /// Bevy sprite sheet bundle
    #[bundle]
    pub sprite: SpriteSheetBundle,
}
