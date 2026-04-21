use bevy::{prelude::*};

use crate::minigames::{
    MinigameState,
    shooting_game::target::{
        advance_expire_and_despawn,
        move_targets,
        maintain_intended_target_count
    },
    shooting_game::level::{setup_minigame_level},
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
                )
            )
            .add_systems(
                Update,
                (
                    maintain_intended_target_count.run_if(in_state(MinigameState::Shoot)),
                    advance_expire_and_despawn.run_if(in_state(MinigameState::Shoot)),
                    move_targets.run_if(in_state(MinigameState::Shoot)),
                )
            );
    
    }
}