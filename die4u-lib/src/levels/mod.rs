use bevy::{app::PluginGroupBuilder, prelude::*};

/// LDtk levels
mod ldtk_level;
/// Procedural generated levels
mod procedural_levels;

pub struct GameLevelsPluginGroup;

impl PluginGroup for GameLevelsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ldtk_level::GameLdtkWorldPlugin)
            .add(procedural_levels::GameProceduralLevelsPlugin)
    }
}
