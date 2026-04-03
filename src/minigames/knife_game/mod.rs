pub mod knife;
pub mod chopping_block;

use bevy::{prelude::*};
use chopping_block::{setup_chopping_block, cleanup_chopping_block};
use knife::{setup_knife, register_keystroke, cut_animation, move_objects, ChoppingGameState, cleanup_knife};
use crate::minigames::shared::score::{setup_score, display_score,cleanup_score,};
use crate::minigames::shared::menu::{setup_menu, listen_keystroke_game, listen_keystroke_menu, GameState};

use crate::minigames::MinigameState;

pub const CHOPPING_BLOCK_WIDTH_PERCENTAGE: f32 = 0.7;
pub const CHOPPING_BLOCK_HEIGHT_PERCENTAGE: f32 = 0.3;
pub const BACKGROUND_Z_INDEX: f32 = 0.;
pub const CUTTABLE_Z_INDEX: f32 = 1.;
pub const MOVABLE_Z_INDEX: f32 = 2.;

pub const KNIFE_WIDTH: f32 = 50.;
pub const KNIFE_X_OFFSET_TO_SHADOW: f32 = 25.0;

pub const SHADOW_START_X_POSITION_PERCENTAGE: f32 = 0.35;
pub const SHADOW_START_Y_POSITION: f32 = 0.;
pub const SHADOW_HEIGHT_PERNCETAGE: f32 = 0.3;
pub const SHADOW_WIDTH: f32 = 3.;

pub const MOVEMENT_SPEED: f32 = 400.;

pub const IMAGE_WIDTH: f32 = 200.;
pub const IMAGE_HEIGHT: f32 = 80.;

pub struct KnifeMinigamePlugin;

impl Plugin for KnifeMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ChoppingGameState::Playing)
            .insert_state(GameState::Play)
            .add_systems(
                OnEnter(MinigameState::Knife),
                (setup_chopping_block, setup_knife, setup_score),
            )
            .add_systems(
                OnExit(MinigameState::Knife),
                (cleanup_chopping_block, cleanup_knife, cleanup_score),
            )
            .add_systems(
    Update,
        (
                    register_keystroke.run_if(in_state(ChoppingGameState::Playing)),
                    cut_animation.run_if(in_state(ChoppingGameState::Cutting)),
                    move_objects.run_if(in_state(ChoppingGameState::Playing))
                ).run_if(in_state(MinigameState::Knife))
            )
            .add_systems(
                Startup,
                setup_menu,
            )
            .add_systems(
                Update,
                listen_keystroke_game.run_if(in_state(GameState::Play))
            )
            .add_systems(
                Update,
                listen_keystroke_menu.run_if(in_state(GameState::Menu))
            )
            .add_systems(Update, display_score.run_if(in_state(MinigameState::Knife)));
    }
}