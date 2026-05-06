pub mod click;
pub mod menu;
pub mod room;
pub mod movable_block;
pub mod cleanup;
pub mod current_level_state;
pub mod interaction;

use bevy::{prelude::*};

use crate::GameState;
use crate::minigames::main::cleanup::cleanup_main_game;
use crate::minigames::main::interaction::check_interaction;
use crate::minigames::main::menu::{exit_game, setup_main_menu, continue_main_game};
use crate::minigames::main::room::{setup_bookshelf, setup_bed, setup_drawer, setup_floor, setup_walls, setup_heaters, setup_table, setup_gift, setup_wall_floor_boundary};
use crate::minigames::main::movable_block::{setup_player, keyboard_input, update_animation};
use crate::minigames::shared::menu::menu_action::MenuAction;
use crate::minigames::shared::menu::state_management::cleanup_menu;
use crate::minigames::{MinigameState};
use crate::minigames::main::click::choose_minigame;
use crate::minigames::shared::menu::user_input::{listen_keystroke_game, listen_keystroke_menu};
use crate::minigames::shared::score::{display_score};
use crate::minigames::main::current_level_state::{KnifeLevel, GunLevel, QuizLevel, setup_gun_level, setup_knife_level, setup_quiz_level};


pub struct MainMinigamePlugin;

impl Plugin for MainMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MinigameState::Main)

            .insert_state(GameState::Play)
            .insert_state(MenuAction::None)
            .add_systems(
                Startup,
                (
                    setup_gun_level,
                    setup_knife_level,
                    setup_quiz_level,
                )
            )
            .add_systems(
                OnEnter(MinigameState::Main),
                (
                    setup_walls,
                    setup_bookshelf,
                    setup_bed,
                    setup_drawer,
                    setup_gift,
                    setup_heaters,
                    setup_table,
                    setup_floor,
                    setup_wall_floor_boundary,
                    setup_player,
                ).chain()
            )
            .add_systems(
                OnExit(MinigameState::Main),
                cleanup_main_game,
            )
            .add_systems(
                Update,
                (
                    // choose_minigame.run_if(in_state(MinigameState::Main))
                    //     .run_if(in_state(GameState::Play)),
                    keyboard_input.run_if(in_state(MinigameState::Main))
                        .run_if(in_state(GameState::Play)),
                    update_animation.run_if(in_state(MinigameState::Main))
                        .run_if(in_state(GameState::Play)),
                    check_interaction.run_if(in_state(MinigameState::Main))
                        .run_if(in_state(GameState::Play)),

                )
            )
            .add_systems(
            Update, 
            display_score.run_if(
                in_state(MinigameState::Knife)
                            .or(in_state(MinigameState::Shoot))
                            .or(in_state(MinigameState::Quiz))
                    )
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