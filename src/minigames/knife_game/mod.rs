pub mod knife;
pub mod chopping_block;
pub mod score;
pub mod level;
pub mod menu;

use bevy::{prelude::*};
use chopping_block::{setup_chopping_block, cleanup_chopping_block};
use knife::{setup_knife, register_keystroke, cut_animation, move_objects, ChoppingGameState, cleanup_knife};
use score::setup_minigame_score;
use level::setup_minigame_level;

use crate::minigames::knife_game::menu::{continue_knife_game, exit_knife_game, restart_knife_game, setup_knife_menu, setup_lose_menu, setup_win_menu};
use crate::minigames::shared::lose::lose_level;
use crate::minigames::shared::menu::menu_action::MenuAction;
use crate::minigames::shared::score::{cleanup_score};
use crate::minigames::shared::level::{cleanup_level};
use crate::minigames::shared::menu::state_management::{GameState, cleanup_menu};



use crate::minigames::MinigameState;
use crate::minigames::shared::win::win_level;

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
            .add_systems(
                // setup things for the minigame
                OnEnter(MinigameState::Knife),
                (
                    setup_minigame_level,
                    setup_minigame_score,
                    setup_chopping_block,
                    setup_knife,
                ).chain(),
            )
            .add_systems(
                // delete things from the minigame after exiting
                OnExit(MinigameState::Knife),
                (
                    cleanup_chopping_block,
                    cleanup_knife,
                    cleanup_score,
                    cleanup_menu,
                    cleanup_level,
                ),
            )
            .add_systems(
                OnEnter(MenuAction::PreLose),
                lose_level,
            )
            .add_systems(
                OnEnter(MenuAction::PreWin),
                win_level,
            )
            .add_systems(
                OnEnter(GameState::Menu),
                (
                    setup_knife_menu.run_if(in_state(ChoppingGameState::Playing))
                        .run_if(in_state(MinigameState::Knife)),
                    setup_lose_menu.run_if(in_state(MenuAction::PreLose))
                        .run_if(in_state(MinigameState::Knife)),
                    setup_win_menu.run_if(in_state(MenuAction::PreWin))
                        .run_if(in_state(MinigameState::Knife)),
                )
            )
            .add_systems(
                OnExit(GameState::Menu),
                cleanup_menu,
            )
            .add_systems(
                // minigame menu controls
                Update,
                (
                    continue_knife_game.run_if(in_state(MenuAction::PreContinue))
                        .run_if(in_state(MinigameState::Knife)),
                    restart_knife_game.run_if(in_state(MenuAction::PreRestart))
                        .run_if(in_state(MinigameState::Knife)),
                    exit_knife_game.run_if(in_state(MenuAction::PreExit))
                        .run_if(in_state(MinigameState::Knife)),
                )
            )
            .add_systems(
                // actual minigame stuff
    Update,
        (
                    register_keystroke.run_if(in_state(ChoppingGameState::Playing)),
                    cut_animation.run_if(in_state(ChoppingGameState::Cutting)),
                    move_objects.run_if(in_state(ChoppingGameState::Playing))
                ).run_if(in_state(MinigameState::Knife))
            );
    }
}