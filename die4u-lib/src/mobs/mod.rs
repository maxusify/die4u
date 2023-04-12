use bevy::{app::PluginGroupBuilder, prelude::*};

use self::player::MobPlayerPlugin;

/// Components for building mobs
pub mod components;

/// Mob module that is base for mob plugins
mod mob;

/// Player plugin
mod player;

/// Plugin group that adds mobs to the game
pub struct GameMobsPluginGroup;

impl PluginGroup for GameMobsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(MobPlayerPlugin)
    }
}
