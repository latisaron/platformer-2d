use bevy::{prelude::*};

use crate::minigames::{
    MinigameState, shared::{level::cleanup_level, lose::lose_level, menu::{menu_action::MenuAction, state_management::{GameState, cleanup_menu}}, score::cleanup_score, win::win_level}, shooting_game::{
        bullets_display::{
            cleanup_bullets_text, display_bullets, setup_bullets
        }, gun::{
            GunAnimationState, animate_gun_in, animate_gun_out, cleanup_gun, gun_follows_mouse, hide_cursor, setup_cursor_icon, restore_default_cursor, setup_gun, show_cursor
        }, level::setup_minigame_level, menu::{continue_shoot_game, exit_shoot_game, restart_shoot_game, setup_lose_menu, setup_shoot_menu, setup_win_menu}, score::setup_minigame_score, target::{
            advance_expire_and_despawn, animate_target, cleanup_targets, listen_for_shots_in_target, maintain_intended_target_count, move_targets, remove_soft_deleted_targets
        }, timer::{cleanup_timer, setup_timer, update_timer},
        environment::{setup_environment, cleanup_environment},
    }
};

pub mod environment;
pub mod gun;
pub mod level;
pub mod menu;
pub mod score;
pub mod target;
pub mod bullets_display;
pub mod timer;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum LossState {
    None,
    Bullets,
    Timer,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ShootingGameState {
    Playing,
    Animating,
}

pub struct ShootingMinigamePlugin;

impl Plugin for ShootingMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(LossState::None)
            .insert_state(ShootingGameState::Playing)
            .insert_state(GunAnimationState::None)
            .add_systems(
            OnEnter(MinigameState::Shoot),
                (
                    setup_minigame_level,
                    setup_minigame_score,
                    setup_cursor_icon,
                    setup_environment,
                    setup_gun,
                    setup_bullets,
                    setup_timer,
                ).chain()
            )
            .add_systems(
                // delete things from the minigame after exiting
                OnExit(MinigameState::Shoot),
                (
                    restore_default_cursor,
                    cleanup_bullets_text,
                    cleanup_environment,
                    cleanup_gun,
                    cleanup_targets,
                    cleanup_menu,
                    cleanup_timer,
                    cleanup_score,
                    cleanup_level,
                ),
            )
            .add_systems(
                OnEnter(MinigameState::Shoot),
                show_cursor,
            )
            .add_systems(
                OnExit(MinigameState::Shoot),
                hide_cursor,
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
                    setup_shoot_menu.run_if(in_state(ShootingGameState::Playing))
                        .run_if(in_state(MenuAction::None))
                        .run_if(in_state(MinigameState::Shoot)),
                    setup_lose_menu.run_if(in_state(MenuAction::PreLose))
                        .run_if(in_state(MinigameState::Shoot)),
                    setup_win_menu.run_if(in_state(MenuAction::PreWin))
                        .run_if(in_state(MinigameState::Shoot)),
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
                    continue_shoot_game.run_if(in_state(MenuAction::PreContinue))
                        .run_if(in_state(MinigameState::Shoot)),
                    restart_shoot_game.run_if(in_state(MenuAction::PreRestart))
                        .run_if(in_state(MinigameState::Shoot)),
                    exit_shoot_game.run_if(in_state(MenuAction::PreExit))
                        .run_if(in_state(MinigameState::Shoot)),
                )
            )
            .add_systems(
                Update,
                (
                    maintain_intended_target_count.run_if(in_state(MinigameState::Shoot)),
                    advance_expire_and_despawn.run_if(in_state(MinigameState::Shoot)),
                    listen_for_shots_in_target.run_if(in_state(MinigameState::Shoot)),
                    move_targets.run_if(in_state(MinigameState::Shoot)),
                    gun_follows_mouse.run_if(in_state(MinigameState::Shoot)),
                    display_bullets.run_if(in_state(MinigameState::Shoot)),
                    update_timer.run_if(in_state(MinigameState::Shoot))
                        .run_if(in_state(MenuAction::None)),
                    animate_gun_out.run_if(in_state(MinigameState::Shoot))
                        .run_if(in_state(GunAnimationState::External)),
                    animate_gun_in.run_if(in_state(MinigameState::Shoot))
                        .run_if(in_state(GunAnimationState::Internal)),
                    animate_target.run_if(in_state(MinigameState::Shoot)),
                    remove_soft_deleted_targets.run_if(in_state(MinigameState::Shoot))
                ).chain()
            );
    
    }
}