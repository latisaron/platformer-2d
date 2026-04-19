pub mod click;
pub mod menu;

use bevy::{prelude::*};

use crate::GameState;
use crate::minigames::main::menu::{exit_game, setup_main_menu, continue_main_game};
use crate::minigames::shared::menu::menu_action::MenuAction;
use crate::minigames::shared::menu::state_management::cleanup_menu;
use crate::minigames::{MinigameState};
use crate::minigames::main::click::choose_minigame;
use crate::minigames::shared::menu::user_input::{listen_keystroke_game, listen_keystroke_menu};
use crate::minigames::shared::score::{display_score};


pub struct MainMinigamePlugin;

impl Plugin for MainMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MinigameState::Main)

            .insert_state(GameState::Play)
            .insert_state(MenuAction::None)
            .add_systems(
                Update,
                choose_minigame
                    .run_if(in_state(MinigameState::Main))
                    .run_if(in_state(GameState::Play))
            )
            .add_systems(
            Update, 
            display_score.run_if(
                in_state(MinigameState::Knife))
            )
            .add_systems(
                Update,
                listen_keystroke_game.run_if(in_state(GameState::Play))
            )
            .add_systems(
                Update,
                listen_keystroke_menu.run_if(in_state(GameState::Menu))
            )
            .add_systems(
                OnEnter(GameState::Menu),
                (
                    setup_main_menu.run_if(in_state(MinigameState::Main)),
                )
            )
            .add_systems(
                // delete things from the minigame after exiting
                OnExit(GameState::Menu),
                (
                            cleanup_menu.run_if(in_state(MinigameState::Main)),
                        ),
            )
            .add_systems(
                Update,
                (
                    exit_game.run_if(in_state(MenuAction::PreExit))
                        .run_if(in_state(MinigameState::Main)),
                    continue_main_game.run_if(in_state(MenuAction::PreContinue))
                        .run_if(in_state(MinigameState::Main)),
                ),
            );
    }
}