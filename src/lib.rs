/// Core functionalities for the game app
pub mod core;

/// Entities (game context)
pub mod entities;

/// Resources
pub mod resources;

// === NEW CRATE ORGANIZATION ===
// Each directory in the `src/` is either `Plugin` or `PluginGroup`
//
// Direcories (Plugin/PluginGroups) are divided into smaller modules
// Those modules have their own ECS items: components, systems and resources
