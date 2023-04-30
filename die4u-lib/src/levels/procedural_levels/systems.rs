use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapTexture;
use bevy_ecs_tilemap::prelude::TilemapId;
use bevy_ecs_tilemap::tiles::{TileBundle, TilePos, TileStorage};
use bevy_ecs_tilemap::TilemapBundle;

use super::assets::ProceduralWorldAssets;
use super::{ChunkManager, CHUNK_SIZE, TILE_SIZE};
use crate::core::MainCamera;

#[derive(Component)]
pub struct WorldChunk;

fn spawn_chunk(
    commands: &mut Commands,
    assets: &Res<ProceduralWorldAssets>,
    chunk_position: IVec2,
) {
    let tilemap_entity = commands.spawn(WorldChunk).id();
    let mut block_storage = TileStorage::empty(CHUNK_SIZE.into());

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let block_position = TilePos { x, y };

            let block_entity = commands
                .spawn((
                    TileBundle {
                        position: block_position,
                        tilemap_id: TilemapId(tilemap_entity),
                        ..default()
                    },
                    // RigidBody::Fixed,
                    // Collider::cuboid(TILE_SIZE.x, TILE_SIZE.y),
                ))
                .id();

            commands.entity(tilemap_entity).add_child(block_entity);
            block_storage.set(&block_position, block_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_position.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_position.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.1,
    ));

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: block_storage,
        texture: TilemapTexture::Single(assets.blocks.clone()),
        tile_size: TILE_SIZE,
        transform,
        ..Default::default()
    });
}

fn camera_pos_to_chunk_pos(camera_position: &Vec2) -> IVec2 {
    let camera_position = camera_position.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);

    camera_position / (chunk_size * tile_size)
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    assets: Res<ProceduralWorldAssets>,
    camera_query: Query<&Transform, With<MainCamera>>,
    mut chunk_manger: ResMut<ChunkManager>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manger.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manger.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &assets, IVec2::new(x, y));
                }
            }
        }
    }
}

pub fn despawn_out_of_range_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<MainCamera>>,
    chunks_query: Query<(Entity, &Transform), With<WorldChunk>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_position = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_position);
            if distance > 320.0 {
                let x = (chunk_position.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_position.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
