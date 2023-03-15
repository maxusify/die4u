use bevy::prelude::*;

pub fn mouse_input(mouse_button_input: Res<Input<MouseButton>>) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        info!("RMB has been pressed.");
    }

    if mouse_button_input.just_pressed(MouseButton::Middle) {
        info!("MMB has been pressed.");
    }
}
