use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::mobs::input::input_map::GamePlayerInput;
use crate::mobs::mob::DefaultMobBundle;
use crate::mobs::PlayerBundle;

use super::assets::PlayerAssets;
use super::components::Player;
use super::input::player_actions::PlayerActions;
use super::movement::process_movement;

pub fn _spawn_default_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    player_input: Res<GamePlayerInput>,
) {
    commands.spawn((
        PlayerBundle {
            mob: DefaultMobBundle {
                sprite: SpriteSheetBundle {
                    texture_atlas: player_assets.idle.clone(),
                    ..Default::default(),
                }
            }
        }
        ));
}

/// Move player with physics
pub fn player_movement_with_physics(
    mut player_query: Query<
        (
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
    player_actions: Query<&ActionState<PlayerActions>, With<Player>>,
    player_assets: Res<PlayerAssets>,
) {
    for (mut velocity, mut texture_atlas_sprite, mut texture_atlas) in player_query.iter_mut() {
        let action_state = player_actions.single();

        let movement_value = process_movement(
            action_state,
            &mut *texture_atlas_sprite,
            &mut *texture_atlas,
            &player_assets,
        );

        velocity.linvel = movement_value * 1000.0;
    }
}


/// Get player input info
pub fn get_player_input_info(
    player_query: Query<&InputMap<PlayerActions>, With<Player>>
) {
    for input_map in player_query.iter() {
        input_map.iter().for_each(|x| {
            debug!("{:#?}", x);
        })
    }
}
