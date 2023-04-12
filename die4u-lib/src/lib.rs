//! Connects all of the game logic

#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;

/// Core game plugin group
/// It contains plugins that spin up the game app itself
mod core;

/// Plugin group that contains everything interactive in the game
/// Like: player, enemies, friendly NPCs, level objects, etc
mod mobs;

/// Plugin group for game levels, maps, etc.
mod levels;

pub use self::core::GameState;

/// Game plugin that groups all of other plugins
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(core::GameCorePluginsGroup);
        app.add_plugins(mobs::GameMobsPluginGroup);
        app.add_plugins(levels::GameLevelsPluginGroup);
    }
}
