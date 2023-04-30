use bevy::{app::PluginGroupBuilder, prelude::*};

/// Procedural generated levels
mod procedural_levels;

/// LDtk levels
mod ldtk;

pub use ldtk::LdtkColliderBundle;

pub struct GameLevelsPluginGroup;

impl PluginGroup for GameLevelsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // .add(ldtk::GameLdtkWorldPlugin)
            .add(procedural_levels::GameProceduralLevelsPlugin)
    }
}
