use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use die4u_rs::core::{GamePluginGroup, GameState};

fn main() {
    let mut app = App::new();

    // States
    app.add_state::<GameState>();

    // Plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(GamePluginGroup);

    // Independent systems
    app.add_system(close_on_esc);

    app.run();
}
