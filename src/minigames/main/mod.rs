pub mod menu;
pub mod room;
pub mod movable_block;
pub mod cleanup;
pub mod current_level_state;
pub mod interaction;
pub mod screen_manager;

use bevy::{prelude::*};

use crate::GameState;
use crate::minigames::main::cleanup::cleanup_main_game;
use crate::minigames::main::interaction::check_interaction;
use crate::minigames::main::menu::{exit_game, setup_main_menu, continue_main_game};
use crate::minigames::main::room::{setup_bookshelf, setup_bed, setup_drawer, setup_floor, setup_walls, setup_heaters, setup_table, setup_gift, setup_wall_floor_boundary};
use crate::minigames::main::movable_block::{setup_player, keyboard_input, update_animation};
use crate::minigames::main::screen_manager::{MainGameState, cleanup_screens, exit_screen, setup_gift_screen, setup_naughty_screen, setup_start_screen};
use crate::minigames::shared::menu::menu_action::MenuAction;
use crate::minigames::shared::menu::state_management::cleanup_menu;
use crate::minigames::{MinigameState};
use crate::minigames::shared::menu::user_input::{listen_keystroke_game, listen_keystroke_menu};
use crate::minigames::shared::score::{display_score};
use crate::minigames::main::current_level_state::{KnifeLevel, GunLevel, QuizLevel, setup_gun_level, setup_knife_level, setup_quiz_level};


pub struct MainMinigamePlugin;

impl Plugin for MainMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MinigameState::Main)

            .insert_state(GameState::Play)
            .insert_state(MenuAction::None)
            .insert_state(MainGameState::StartScreen)
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
                        .run_if(in_state(GameState::Play))
                        .run_if(in_state(MainGameState::PlayScreen)),
                    update_animation.run_if(in_state(MinigameState::Main))
                        .run_if(in_state(GameState::Play))
                        .run_if(in_state(MainGameState::PlayScreen)),
                    check_interaction.run_if(in_state(MinigameState::Main))
                        .run_if(in_state(GameState::Play))
                        .run_if(in_state(MainGameState::PlayScreen)),

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
            )
            .add_systems(
                OnEnter(MainGameState::StartScreen),
                setup_start_screen
            )
            .add_systems(
                OnEnter(MainGameState::NaughtyScreen),
                setup_naughty_screen,
            )
            .add_systems(
                OnEnter(MainGameState::GiftScreen),
                setup_gift_screen,
            )
            .add_systems(
                OnExit(MainGameState::StartScreen),
                cleanup_screens,
            )
            .add_systems(
                OnExit(MainGameState::NaughtyScreen),
                cleanup_screens,
            )
            .add_systems(
                OnExit(MainGameState::GiftScreen),
                cleanup_screens,
            )
            .add_systems(
                Update,
                exit_screen.run_if(in_state(MinigameState::Main))
                        .run_if(in_state(GameState::Play))
                        .run_if(in_state(MainGameState::StartScreen).or(in_state(MainGameState::GiftScreen)).or(in_state(MainGameState::NaughtyScreen))),
            );

    }
}