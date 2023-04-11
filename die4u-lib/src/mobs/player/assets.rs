use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas(tile_size_x = 120., tile_size_y = 80., columns = 10, rows = 1))]
    #[asset(path = "images/player/player_idle.png")]
    pub sprite_idle: Handle<TextureAtlas>,
}
