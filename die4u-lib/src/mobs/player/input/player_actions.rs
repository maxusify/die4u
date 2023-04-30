use leafwing_input_manager::Actionlike;

/// Defines possible player actions with inputs
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerActions {
    /// Moving left
    MoveLeft,
    /// Moving right
    MoveRight,
    /// Jumping
    Jump,
    /// Falling
    Fall,
    /// Extracting
    Extract,
    /// Placing objects
    Place,
}
