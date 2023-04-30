use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// Resource handling player assets
#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    /// Handle for idle sprite
    #[asset(key = "player.idle")]
    pub idle: Handle<TextureAtlas>,

    /// Handle for walking sprite
    #[asset(key = "player.walk")]
    pub walk: Handle<TextureAtlas>,

    /// Handle for running sprite
    #[asset(key = "player.run")]
    pub run: Handle<TextureAtlas>,
}
