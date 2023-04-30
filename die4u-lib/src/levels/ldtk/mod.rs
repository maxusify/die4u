use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::plugin::PhysicsSet;

use crate::levels::ldtk::wall::{spawn_wall_collision, LdtkWallBundle};
use crate::mobs::{Player, PlayerBundle};
use crate::GameState;

/// Setup LDtk related stuff
mod setup;
/// Logic for walls
mod wall;

pub use setup::LdtkColliderBundle;

/// Resource for LDtk world
#[derive(AssetCollection, Resource)]
pub struct LdtkAssets {
    #[asset(path = "levels/world.ldtk")]
    pub ldtk_levels: Handle<LdtkAsset>,
}

/// Plugin that adds levels to the game
pub struct GameLdtkWorldPlugin;
impl Plugin for GameLdtkWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin);
        app.configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend));
        app.add_collection_to_loading_state::<_, LdtkAssets>(GameState::AssetLoading);
        app.insert_resource(LevelSelection::Index(0));
        app.insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        });
        app.add_system(init_level_world.in_schedule(OnEnter(GameState::Playing)));
        app.add_systems(
            (spawn_wall_collision, update_level_selection).in_set(OnUpdate(GameState::Playing)),
        );
        app.register_ldtk_int_cell::<LdtkWallBundle>(2);
        app.register_ldtk_entity::<PlayerBundle>("Player");
    }
}

/// Initialize LDtk world
fn init_level_world(mut commands: Commands, assets: Res<LdtkAssets>) {
    info!("Initialized LDtk world");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.ldtk_levels.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(2.0, 2.0, 2.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level_bounds = Rect {
                min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
                max: Vec2::new(
                    level_transform.translation.x + ldtk_level.level.px_wid as f32,
                    level_transform.translation.y + ldtk_level.level.px_hei as f32,
                ),
            };

            for player_transform in &player_query {
                if player_transform.translation.x < level_bounds.max.x
                    && player_transform.translation.x > level_bounds.min.x
                    && player_transform.translation.y < level_bounds.max.y
                    && player_transform.translation.y > level_bounds.min.y
                    && !level_selection.is_match(&0, &ldtk_level.level)
                {
                    *level_selection = LevelSelection::Iid(ldtk_level.level.iid.clone());
                }
            }
        }
    }
}
