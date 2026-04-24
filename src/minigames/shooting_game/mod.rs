use bevy::{prelude::*};

use crate::minigames::{
    MinigameState,
    shooting_game::{
        bullets_display::{
            display_bullets, setup_bullets
        }, gun::{
            gun_follows_mouse, cursor_visibility_system, setup_cursor_icon, setup_gun,
        }, level::setup_minigame_level, score::setup_minigame_score, target::{
            advance_expire_and_despawn, listen_for_shots_in_target, maintain_intended_target_count, move_targets
        }
    }
};

pub mod environment;
pub mod gun;
pub mod level;
pub mod menu;
pub mod score;
pub mod target;
pub mod bullets_display;

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
                    setup_bullets,
                ).chain()
            )
            .add_systems(
                Update,
                (
                    cursor_visibility_system.run_if(in_state(MinigameState::Shoot)),
                    maintain_intended_target_count.run_if(in_state(MinigameState::Shoot)),
                    advance_expire_and_despawn.run_if(in_state(MinigameState::Shoot)),
                    listen_for_shots_in_target.run_if(in_state(MinigameState::Shoot)),
                    move_targets.run_if(in_state(MinigameState::Shoot)),
                    gun_follows_mouse.run_if(in_state(MinigameState::Shoot)),
                    display_bullets.run_if(in_state(MinigameState::Shoot)),
                ).chain()
            );
    
    }
}