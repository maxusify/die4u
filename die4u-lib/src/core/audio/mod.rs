use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

/// Groups logic for game audio
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin);
    }
}
