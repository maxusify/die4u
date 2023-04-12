use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(key = "player.idle")]
    pub idle: Handle<TextureAtlas>,
    #[asset(key = "player.walk")]
    pub walk: Handle<TextureAtlas>,
    #[asset(key = "player.run")]
    pub run: Handle<TextureAtlas>,
}
