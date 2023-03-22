use bevy::prelude::*;

/// Gamepad input systems
mod gamepad;

/// Keyboard input systems
mod keyboard;

/// Mouse input systems
mod mouse;

/// Plugin grouping all input systems
pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            gamepad::gamepad_input,
            keyboard::keyboard_input,
            mouse::mouse_input,
        ));
    }
}
