use bevy::prelude::*;

// TODO: Reimagine inventory system

/// Inventory slots counter
#[derive(Clone, Component, Default)]
pub struct InventorySlots(i32);

/// Makes complete inventory mechanic for entity
#[derive(Clone, Default, Bundle)]
pub struct InventoryBundle {
    /// Inventory slots count
    pub slot_count: InventorySlots,
}
