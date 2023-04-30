use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// Resource handling procedural worlds assets
#[derive(AssetCollection, Resource)]
pub struct ProceduralWorldAssets {
    /// Regular blocks
    #[asset(key = "blocks")]
    pub blocks: Handle<Image>,
}
