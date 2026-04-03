pub mod knife_game;
pub mod shared;

use bevy::{prelude::*};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MinigameState {
    Knife,
}