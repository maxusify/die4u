use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::core::GameState;

#[derive(AssetCollection, Resource)]
pub struct LdtkAssets {
    #[asset(path = "levels/world.ldtk")]
    pub ldtk_levels: Handle<LdtkAsset>
}

pub struct GameLevelsWorldPlugin;

impl Plugin for GameLevelsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin);
        app.add_collection_to_loading_state::<_, LdtkAssets>(GameState::AssetLoading);
        app.insert_resource(LevelSelection::Index(0));
        app.add_system(init_level_world.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn init_level_world(mut commands: Commands, assets: Res<LdtkAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.ldtk_levels.clone(),
        transform: Transform {
            scale: Vec3::new(3.0, 3.0, 3.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub struct GameLevelsPluginGroup;

impl PluginGroup for GameLevelsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GameLevelsWorldPlugin)
    }
}
