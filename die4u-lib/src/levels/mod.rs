use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::GameState;

/// Resource for LDtk world
#[derive(AssetCollection, Resource)]
pub struct LdtkAssets {
    #[asset(path = "levels/world.ldtk")]
    pub ldtk_levels: Handle<LdtkAsset>,
}

/// Plugin that adds levels to the game
pub struct GameLevelsWorldPlugin;
impl Plugin for GameLevelsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin);
        app.add_collection_to_loading_state::<_, LdtkAssets>(GameState::AssetLoading);
        app.insert_resource(LevelSelection::Index(0));
        app.add_system(init_level_world.in_schedule(OnEnter(GameState::Playing)));
    }
}

/// Initialize LDtk world
fn init_level_world(mut commands: Commands, assets: Res<LdtkAssets>) {
    info!("Initialized LDtk world");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.ldtk_levels.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(3.0, 3.0, 3.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub struct GameLevelsPluginGroup;

impl PluginGroup for GameLevelsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(GameLevelsWorldPlugin)
    }
}
