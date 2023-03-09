use std::fmt::Display;

use bevy::prelude::*;

/// Identifies that entity is a mob
#[derive(Component, Default)]
pub struct Mob;

/// Name for the mob
#[derive(Component)]
pub struct Name(String);
impl Default for Name {
    fn default() -> Self {
        Self("Unknown".to_string())
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Health points
#[derive(Component, Default)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

/// Shield points
#[derive(Component, Default)]
pub struct Shield {
    pub current: i32,
    pub max: i32,
}

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

/// Equipment
pub struct Equipment {
    /// Number of slots that can store items
    pub slots: i32,
    pub slots_content: Vec<String>,
}

/// Bundle for creating basic mob with health and shield
#[derive(Bundle, Default)]
pub struct DefaultMobBundle {
    pub name: Name,
    pub transform: Transform,
    pub hp: Health,
    pub sp: Shield,
    _mob: Mob,
}

pub fn spawn_basic_mob(mut commands: Commands) {
    commands.spawn(DefaultMobBundle {
        name: Name("Prinz".to_owned()),
        ..Default::default()
    });
    commands.spawn(DefaultMobBundle {
        name: Name("Roza".to_owned()),
        ..Default::default()
    });
    commands.spawn(DefaultMobBundle {
        name: Name("Oguri".to_owned()),
        ..Default::default()
    });
}

pub fn welcome_basic_mob(query: Query<(&Name, &Health)>) {
    for (name, hp) in query.iter() {
        println!("Welcome {}! You have {} HP.", name, hp.current);
    }
}
