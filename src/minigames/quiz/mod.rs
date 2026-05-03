use bevy::{prelude::*};

use crate::minigames::{
    MinigameState,
    quiz::{
        inventory::{
            basic::{handle_inventory_clicks, setup_inventory_ui}, category_popup::handle_item_selection, player_model::setup_player_model
        },
        level::setup_minigame_level, menu::{continue_quiz_game, exit_quiz_game, setup_quiz_menu, setup_win_menu}, password_popup::{
            alarm::start_alarm, cleanup::cleanup_password_popup_entities, close::{
                handle_close_click, setup_close_button
            }, error::{despawn_error_if_done, setup_error_flash_popup}, mumbo_jumbo::{
                animate_mumbo_jumbo,
                setup_mumbo_jumbo
            }, password::setup_password, popup::{
                handle_popup_input, setup_password_popup
            }, submit::{
                handle_submit_click, setup_submit_button
            }, win::{despawn_win_if_done, setup_win_flash_popup}
        }, request_review_button::{
            cleanup_request_review_button,
            handle_request_review_button_interaction,
            setup_review_request_button
        }, score::setup_minigame_score
    }, shared::{level::cleanup_level, menu::{menu_action::MenuAction, state_management::{GameState, cleanup_menu}}, score::cleanup_score}
};

pub mod request_review_button;
pub mod inventory;
pub mod password_popup;
pub mod level;
pub mod menu;
pub mod score;

#[derive(States, Hash, PartialEq, Eq, Debug, Clone)]
pub enum QuizGameState {
    None,
    PasswordPopup,
    PasswordPopupError,
    PasswordPopupWin,
    Choosing,
    Browsing,
}

pub struct QuizMinigamePlugin;

impl Plugin for QuizMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(QuizGameState::Choosing)
            .add_systems(
            OnEnter(MinigameState::Quiz),
                (
                    setup_minigame_level,
                    setup_minigame_score,
                    setup_review_request_button,
                    setup_player_model,
                    setup_inventory_ui,
                ).chain()
            )
            .add_systems(
                OnExit(MinigameState::Quiz),
                (
                    cleanup_request_review_button,
                    cleanup_level,
                    cleanup_score,
                ),
            )
            .add_systems(
                Update,
                    (
                    handle_request_review_button_interaction.run_if(in_state(MinigameState::Quiz))
                            .run_if(in_state(QuizGameState::Choosing)),
                    handle_inventory_clicks.run_if(in_state(MinigameState::Quiz))
                            .run_if(in_state(QuizGameState::Choosing)),
                    handle_item_selection.run_if(in_state(MinigameState::Quiz))
                            .run_if(in_state(QuizGameState::Browsing)),
                    )
            )
            .add_systems(
                OnEnter(QuizGameState::PasswordPopup),
                (
                    start_alarm,
                    setup_password,
                    setup_password_popup,
                    setup_submit_button,
                    setup_close_button,
                    setup_mumbo_jumbo
                )
            )
            .add_systems(
                OnExit(QuizGameState::PasswordPopup),
                (
                    cleanup_password_popup_entities,
                )
            )
            .add_systems(
                Update,
                    (
                            animate_mumbo_jumbo.run_if(in_state(MinigameState::Quiz))
                                .run_if(in_state(QuizGameState::PasswordPopup)),
                            handle_popup_input.run_if(in_state(MinigameState::Quiz))
                                .run_if(in_state(QuizGameState::PasswordPopup)),
                            handle_submit_click.run_if(in_state(MinigameState::Quiz))
                                .run_if(in_state(QuizGameState::PasswordPopup)),
                            handle_close_click.run_if(in_state(MinigameState::Quiz))
                                .run_if(in_state(QuizGameState::PasswordPopup)),
                    )
            )
            .add_systems(
                OnEnter(QuizGameState::PasswordPopupError),
                setup_error_flash_popup,
            ).add_systems(
                Update,
                despawn_error_if_done.run_if(in_state(MinigameState::Quiz))
                                .run_if(in_state(QuizGameState::PasswordPopupError)),
            )
            .add_systems(
                OnEnter(QuizGameState::PasswordPopupWin),
                setup_win_flash_popup,
            ).add_systems(
                Update,
                despawn_win_if_done.run_if(in_state(MinigameState::Quiz))
                                .run_if(in_state(QuizGameState::PasswordPopupWin)),
            ).add_systems(
                OnEnter(GameState::Menu),
                (
                    setup_quiz_menu.run_if(in_state(MenuAction::None))
                        .run_if(in_state(MinigameState::Quiz)),
                    setup_win_menu.run_if(in_state(MenuAction::PreWin))
                        .run_if(in_state(MinigameState::Quiz)),
                )
            )
            .add_systems(
                Update,
                (
                    continue_quiz_game.run_if(in_state(MenuAction::PreContinue))
                        .run_if(in_state(MinigameState::Quiz)),
                    exit_quiz_game.run_if(in_state(MenuAction::PreExit))
                        .run_if(in_state(MinigameState::Quiz)),
                )
            )
            .add_systems(
                OnExit(GameState::Menu),
                cleanup_menu,
            );


            // .add_systems(
            //     OnEnter(MinigameState::Quiz),
            //     show_cursor,
            // )
            // .add_systems(
            //     OnExit(MinigameState::Quiz),
            //     hide_cursor,
            // )
            // .add_systems(
            //     OnEnter(MenuAction::PreLose),
            //     lose_level,
            // )
            // .add_systems(
            //     OnEnter(MenuAction::PreWin),
            //     win_level,
            // )
            // .add_systems(
            //     OnEnter(GameState::Menu),
            //     (
            //         setup_shoot_menu.run_if(in_state(QuizingGameState::Playing))
            //             .run_if(in_state(MenuAction::None))
            //             .run_if(in_state(MinigameState::Quiz)),
            //         setup_lose_menu.run_if(in_state(MenuAction::PreLose))
            //             .run_if(in_state(MinigameState::Quiz)),
            //         setup_win_menu.run_if(in_state(MenuAction::PreWin))
            //             .run_if(in_state(MinigameState::Quiz)),
            //     )
            // )
            // .add_systems(
            //     OnExit(GameState::Menu),
            //     cleanup_menu,
            // )
            // .add_systems(
            //     // minigame menu controls
            //     Update,
            //     (
            //         continue_shoot_game.run_if(in_state(MenuAction::PreContinue))
            //             .run_if(in_state(MinigameState::Quiz)),
            //         restart_shoot_game.run_if(in_state(MenuAction::PreRestart))
            //             .run_if(in_state(MinigameState::Quiz)),
            //         exit_shoot_game.run_if(in_state(MenuAction::PreExit))
            //             .run_if(in_state(MinigameState::Quiz)),
            //     )
            // )
            // .add_systems(
            //     Update,
            //     (
            //         maintain_intended_target_count.run_if(in_state(MinigameState::Quiz)),
            //         advance_expire_and_despawn.run_if(in_state(MinigameState::Quiz)),
            //         listen_for_shots_in_target.run_if(in_state(MinigameState::Quiz)),
            //         move_targets.run_if(in_state(MinigameState::Quiz)),
            //         gun_follows_mouse.run_if(in_state(MinigameState::Quiz)),
            //         display_bullets.run_if(in_state(MinigameState::Quiz)),
            //         update_timer.run_if(in_state(MinigameState::Quiz))
            //             .run_if(in_state(MenuAction::None)),
            //         animate_gun_out.run_if(in_state(MinigameState::Quiz))
            //             .run_if(in_state(GunAnimationState::External)),
            //         animate_gun_in.run_if(in_state(MinigameState::Quiz))
            //             .run_if(in_state(GunAnimationState::Internal)),
            //         animate_target.run_if(in_state(MinigameState::Quiz)),
            //         remove_soft_deleted_targets.run_if(in_state(MinigameState::Quiz))
            //     ).chain()
            // );
    
    }
}