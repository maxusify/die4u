use bevy::{prelude::*, window::close_on_esc};
use bevy_asset_loader::prelude::*;
use die4u_rs::{
    core::{GameCorePluginsGroup, GameState},
    levels::GameLevelsPluginGroup,
    mobs::GameMobsPluginGroup,
};

fn main() {
    let mut app = App::new();

    // States
    app.add_state::<GameState>();

    // Loading state for assets
    app.add_loading_state(LoadingState::new(GameState::AssetLoading));

    // Plugins
    app.add_plugins(GameCorePluginsGroup)
        .add_plugins(GameMobsPluginGroup)
        .add_plugins(GameLevelsPluginGroup);

    // Independent systems
    app.add_system(close_on_esc);

    app.run();
}
