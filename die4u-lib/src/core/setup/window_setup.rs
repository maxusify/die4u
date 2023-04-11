use bevy::window::{PresentMode, Window, WindowResolution};

const WINDOW_TITLE: &str = "DiE4u";
const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;
const WINDOW_PRESENT_MODE: PresentMode = PresentMode::AutoVsync;

pub fn initialize_window() -> Option<Window> {
    Some(Window {
        title: WINDOW_TITLE.to_string(),
        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        present_mode: WINDOW_PRESENT_MODE,
        ..Default::default()
    })
}
