use crate::mobs::input::player_actions::PlayerActions;
use crate::mobs::player::assets::PlayerAssets;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

/// Processes movement actions and returns direction vector
pub fn process_movement(
    action_state: &ActionState<PlayerActions>,
    texture_atlas_sprite: &mut TextureAtlasSprite,
    texture_atlas: &mut Handle<TextureAtlas>,
    player_assets: &Res<PlayerAssets>,
) -> Vec2 {
    let mut calculated_direction: Vec2 = Vec2::splat(0.0);

    let _ = action_state
        .get_pressed()
        .iter()
        .map(|action| match action {
            PlayerActions::MoveRight => {
                calculated_direction.x = 1.0;
                texture_atlas_sprite.flip_x = false;
                *texture_atlas = player_assets.walk.clone()
            }
            PlayerActions::MoveLeft => {
                calculated_direction.x = -1.0;
                texture_atlas_sprite.flip_x = true;
                *texture_atlas = player_assets.walk.clone()
            }
            PlayerActions::Jump => {
                calculated_direction.y = 1.0;
            }
            _ => (),
        });

    calculated_direction
}
