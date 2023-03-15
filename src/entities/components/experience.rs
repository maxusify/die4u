use bevy::prelude::*;

/// Experience levels
#[derive(Component, Default)]
pub struct ExperienceLevels {
    pub min: i32,
    pub current: i32,
    pub max: i32,
}

/// Experience points
#[derive(Component, Default)]
pub struct ExperiencePoints {
    pub min: i32,
    pub current: i32,
    pub max: i32,
}

/// Experience mechanic
#[derive(Bundle, Default)]
pub struct Experience {
    pub exp_points: ExperiencePoints,
    pub exp_levels: ExperienceLevels,
}
