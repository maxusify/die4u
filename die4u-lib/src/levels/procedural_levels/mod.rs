use crate::levels::procedural_levels::assets::ProceduralWorldAssets;
use crate::levels::procedural_levels::systems::{
    despawn_out_of_range_chunks, spawn_chunks_around_camera,
};
use crate::GameState;
use bevy::app::{App, Plugin};
use bevy::math::{IVec2, UVec2};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::map::{TilemapRenderSettings, TilemapTileSize};

/// Assets for procedural worlds
mod assets;
/// Logic for procedural worlds
mod systems;

/// Tilemap tile size
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
/// Size of a chunk
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };
/// Render size of a chunk
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x,
    y: CHUNK_SIZE.y,
};

/// Manages spawned chunks
#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

pub struct GameProceduralLevelsPlugin;

impl Plugin for GameProceduralLevelsPlugin {
    fn build(&self, app: &mut App) {
        // Assets
        app.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::AssetLoading,
            "levels.assets.ron",
        )
        .add_collection_to_loading_state::<_, ProceduralWorldAssets>(GameState::AssetLoading);

        // Tilemap
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        });

        // Chunk manager
        app.insert_resource(ChunkManager::default());

        // Systems for spawning and despawning
        app.add_system(spawn_chunks_around_camera.in_set(OnUpdate(GameState::Playing)));
        app.add_system(despawn_out_of_range_chunks.in_set(OnUpdate(GameState::Playing)));
    }
}
