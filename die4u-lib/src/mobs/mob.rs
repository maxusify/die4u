use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::Health;
use crate::levels::LdtkColliderBundle;

/// Identifies that entity is a mob
#[derive(Clone, Default, Component)]
pub struct Mob;

/// Statistics
#[derive(Clone, Default, Bundle)]
pub struct StatisticsBundle {
    /// Health points
    pub health: Health,
}

/// Bundle for creating basic mob with health and shield
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DefaultMobBundle {
    /// External force affecting mob
    pub external_force: ExternalForce,
    /// External impulse affecting mob
    pub external_impulse: ExternalImpulse,
    /// Indicates that entity is a child of LDtk level
    pub worldly: Worldly,

    /// Statistics
    #[bundle]
    pub stats: StatisticsBundle,

    /// Sprite
    #[bundle]
    pub sprite: SpriteSheetBundle,

    /// Collider for physics
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: LdtkColliderBundle,

    /// LDtk instance of entity
    #[from_entity_instance]
    entity_instance: EntityInstance,

    /// Marker component indicating that entity is a mob
    _mob: Mob,
}
