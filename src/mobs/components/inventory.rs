use bevy::prelude::*;

#[derive(Component, Default)]
pub struct InventorySlots(i32);

#[derive(Bundle, Default)]
pub struct InventoryBundle {
    pub slot_count: InventorySlots,
}
