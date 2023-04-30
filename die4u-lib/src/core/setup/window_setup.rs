use bevy::window::{PresentMode, Window, WindowResolution};

/// Title of the game window
const WINDOW_TITLE: &str = "DiE4u";
/// Game window width
const WINDOW_WIDTH: f32 = 1280.;
/// Game window height
const WINDOW_HEIGHT: f32 = 720.;
/// Game window present mode
const WINDOW_PRESENT_MODE: PresentMode = PresentMode::AutoVsync;

/// Used by WindowPlugin for initializing game window
pub fn initialize_window() -> Option<Window> {
    Some(Window {
        title: WINDOW_TITLE.to_string(),
        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        present_mode: WINDOW_PRESENT_MODE,
        ..Default::default()
    })
}
