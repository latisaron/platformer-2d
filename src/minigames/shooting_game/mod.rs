use bevy::{prelude::*};

use crate::minigames::{MinigameState, shooting_game::target::{move_targets, spawn_targets}};

pub mod environment;
pub mod gun;
pub mod menu;
pub mod score;
pub mod target;

pub struct ShootingMinigamePlugin;

impl Plugin for ShootingMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MinigameState::Shoot),
            spawn_targets,
            )
            .add_systems(
                Update,
                move_targets.run_if(in_state(MinigameState::Shoot)),
            );
    
    }
}