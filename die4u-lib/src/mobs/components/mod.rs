/// Experience component
mod experience;
/// Health component
mod health;
/// Inventory component
mod inventory;
/// Name component
mod name;
/// Shield component
mod shield;

pub use self::{
    experience::Experience, health::Health, inventory::InventoryBundle, name::Name, shield::Shield,
};
