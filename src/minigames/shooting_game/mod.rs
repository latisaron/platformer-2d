use bevy::{prelude::*};

use crate::minigames::{
    MinigameState,
    shared::menu::state_management::GameState,
    shooting_game::{
        gun::{
            setup_cursor_icon,
            hide_cursor,
            show_cursor,
            setup_gun,
        },
        level::setup_minigame_level,
        score::setup_minigame_score,
        target::{
            advance_expire_and_despawn,
            maintain_intended_target_count,
            listen_for_shots_in_target,
            move_targets,
        },
    }
};

pub mod environment;
pub mod gun;
pub mod level;
pub mod menu;
pub mod score;
pub mod target;

pub struct ShootingMinigamePlugin;

impl Plugin for ShootingMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MinigameState::Shoot),
                (
                    setup_minigame_level,
                    setup_minigame_score,
                    setup_cursor_icon,
                    setup_gun,
                ).chain()
            )
            .add_systems(
                OnEnter(GameState::Menu),
                        hide_cursor.run_if(in_state(MinigameState::Shoot))
            )
            .add_systems(
                OnExit(GameState::Menu),
                        show_cursor.run_if(in_state(MinigameState::Shoot))
            )
            .add_systems(
                Update,
                (
                    maintain_intended_target_count.run_if(in_state(MinigameState::Shoot)),
                    advance_expire_and_despawn.run_if(in_state(MinigameState::Shoot)),
                    listen_for_shots_in_target.run_if(in_state(MinigameState::Shoot)),
                    move_targets.run_if(in_state(MinigameState::Shoot)),
                ).chain()
            );
    
    }
}