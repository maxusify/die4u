use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{close_on_esc, WindowResolution},
};
use bevy_asset_loader::prelude::*;
use die4u_rs::{
    core::{
        setup::window_setup::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH},
        GameCorePluginsGroup, GameState,
    },
    mobs::GameMobsPluginGroup,
};

fn main() {
    let mut app = App::new();

    // States
    app.add_state::<GameState>();

    // Loading state for assets
    app.add_loading_state(LoadingState::new(GameState::AssetLoading));

    // Plugins
    app
        // Default Bevy plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.to_string(),
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // Diagnostics for progress plugin
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Game core plugins
        .add_plugins(GameCorePluginsGroup)
        .add_plugins(GameMobsPluginGroup);

    // Independent systems
    app.add_system(close_on_esc);

    app.run();
}
