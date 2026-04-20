pub mod main;
pub mod knife_game;
pub mod shared;
pub mod shooting_game;

use bevy::{prelude::*};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MinigameState {
    Main,
    Knife,
    Shoot,
}