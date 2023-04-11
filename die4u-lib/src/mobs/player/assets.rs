use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(key = "player.idle")]
    pub player_idle: Handle<TextureAtlas>,
}
