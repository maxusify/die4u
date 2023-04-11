use bevy::prelude::*;

use die4u_lib::{GamePlugin, GameState};

fn main() {
    let mut app = App::new();
    // States
    app.add_state::<GameState>();
    app.add_plugin(GamePlugin);
    app.run();
}
