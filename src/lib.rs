// === NEW CRATE ORGANIZATION ===
// Each directory in the `src/` is either `Plugin` or `PluginGroup`
//
// Direcories (Plugin/PluginGroups) are divided into smaller modules
// Those modules have their own ECS items: components, systems and resources

/// Core game plugin group
/// It cotains plugins that spin up the game app itself
pub mod core;

/// Plugin group that contains everything interactable in the game
/// Like: player, enemies, friendly NPCs, interactive level objects, etc
pub mod mobs;

/// Plugin group for game levels, maps, etc.
pub mod levels;
