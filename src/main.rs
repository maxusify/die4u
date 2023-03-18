use bevy::{prelude::*, window::close_on_esc};
use bevy_asset_loader::prelude::*;
use die4u_rs::{
    core::{setup::window_setup::initialize_window, GameCorePluginsGroup, GameState},
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
                    primary_window: initialize_window(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // Game core plugins
        .add_plugins(GameCorePluginsGroup)
        .add_plugins(GameMobsPluginGroup);

    // Independent systems
    app.add_system(close_on_esc);

    app.run();
}
