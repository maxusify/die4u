use bevy::prelude::*;

/// Experience levels
#[derive(Clone, Component, Default)]
pub struct ExperienceLevels {
    /// Minimal experience level
    pub min: i32,
    /// Current experience level
    pub current: i32,
    /// Maximum experience level
    pub max: i32,
}

/// Experience points
#[derive(Clone, Component, Default)]
pub struct ExperiencePoints {
    /// Minimal amaount of experience points
    pub min: i32,
    /// Current amount of experience points
    pub current: i32,
    /// Maximum amount of experience points (when level-up should occur)
    pub max: i32,
}

/// Experience mechanic
#[derive(Clone, Bundle, Default)]
pub struct Experience {
    /// Experience points counter
    pub exp_points: ExperiencePoints,
    /// Experience level counter
    pub exp_levels: ExperienceLevels,
}
