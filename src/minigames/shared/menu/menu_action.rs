use bevy::{prelude::*};

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum MenuAction {
    None,
    // controllable from the user
    PreContinue,
    PreRestart,
    PreExit,
    // auto-controlled by the game
    PreLose,
    PreWin,
}