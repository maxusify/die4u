use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ProceduralWorldAssets {
    #[asset(key = "blocks")]
    pub blocks: Handle<Image>,
}
