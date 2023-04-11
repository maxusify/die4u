use bevy::prelude::Component;
use std::fmt::Display;

/// Name for the mob
#[derive(Component)]
pub struct Name(pub String);

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
