use leafwing_input_manager::Actionlike;

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
