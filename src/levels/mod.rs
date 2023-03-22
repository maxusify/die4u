use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GameLevelsPluginGroup;

impl PluginGroup for GameLevelsPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
